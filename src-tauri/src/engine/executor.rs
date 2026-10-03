use std::collections::HashMap;
use std::future::Future;
use std::time::{Duration, Instant};

use tauri::ipc::Channel;

use crate::error::AppError;
use crate::nodes::registry::NodeRegistry;
use crate::types::*;

use super::context::{CancelToken, ExecutionContext};
use super::graph::FlowGraph;

pub struct Engine {
    registry: NodeRegistry,
}

/// How often an in-flight node checks whether the run was cancelled.
const CANCEL_POLL_INTERVAL: Duration = Duration::from_millis(25);

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

impl Engine {
    pub fn new() -> Self {
        Self {
            registry: NodeRegistry::new(),
        }
    }

    pub async fn execute(
        &self,
        doc: &FlowDocument,
        channel: &Channel<ExecutionEvent>,
        cancel: CancelToken,
    ) -> Result<ExecutionResult, AppError> {
        let start = Instant::now();
        let flow_graph = FlowGraph::from_document(doc)?;

        let ctx = ExecutionContext::with_cancel_token(cancel);

        let node_map: HashMap<String, &FlowNode> =
            doc.nodes.iter().map(|n| (n.id.clone(), n)).collect();

        let mut node_results: HashMap<String, NodeResult> = HashMap::new();
        let mut had_error = false;

        for layer in &flow_graph.execution_layers {
            if ctx.is_cancelled() {
                return Err(AppError::Cancelled);
            }

            for node_id in layer {
                if ctx.is_cancelled() {
                    return Err(AppError::Cancelled);
                }

                let node = node_map
                    .get(node_id)
                    .ok_or_else(|| AppError::Graph(format!("Node {} not found", node_id)))?;

                let executor =
                    self.registry
                        .get(&node.node_type)
                        .ok_or_else(|| AppError::NodeExecution {
                            node_id: node_id.clone(),
                            message: format!("Unknown node type: {}", node.node_type),
                        })?;

                // Gather inputs from upstream node outputs
                let input_edges = flow_graph.get_input_edges(node_id);
                let mut inputs = HashMap::new();

                for edge in &input_edges {
                    let handle = edge.target_handle.as_deref().unwrap_or("input");
                    let source_handle = edge.source_handle.as_deref().unwrap_or("value");
                    let value = ctx.get_input(&edge.source, source_handle).await;
                    inputs.insert(handle.to_string(), value);

                    if let Some(result) = node_results.get(&edge.source) {
                        if !result.success {
                            if let Some(error) = &result.error {
                                inputs.insert(
                                    "__upstreamError".to_string(),
                                    NodeValue::String(error.clone()),
                                );
                            }
                        }
                    }
                }

                let _ = channel.send(ExecutionEvent::NodeStarted {
                    node_id: node_id.clone(),
                });

                // Set current node ID for error context
                ctx.set_current_node_id(Some(node_id.clone())).await;

                let node_start = Instant::now();
                let node_future = executor.execute(inputs, node.data.clone(), &ctx);
                let result = if runs_to_completion(&node.node_type) {
                    node_future.await
                } else {
                    let Some(result) = run_until_cancelled(node_future, &ctx.cancelled).await
                    else {
                        ctx.set_current_node_id(None).await;
                        return Err(AppError::Cancelled);
                    };
                    result
                };
                let duration_ms = node_start.elapsed().as_millis() as u64;

                // Clear current node ID after execution
                ctx.set_current_node_id(None).await;

                // A synchronous node (e.g. an interrupted JS script) or a
                // run-to-completion node can finish after Stop; report the run
                // as cancelled rather than as a node failure.
                if ctx.is_cancelled() {
                    return Err(AppError::Cancelled);
                }

                match result {
                    Ok(outputs) => {
                        let preview = outputs
                            .values()
                            .next()
                            .map(|v| v.preview(200))
                            .unwrap_or_default();

                        // Serialize full output data (cap at 50KB)
                        let output_data = {
                            let json_map: serde_json::Map<String, serde_json::Value> = outputs
                                .iter()
                                .map(|(k, v)| (k.clone(), v.to_json_value()))
                                .collect();
                            let val = serde_json::Value::Object(json_map);
                            let serialized = serde_json::to_string(&val).unwrap_or_default();
                            if serialized.len() <= 50_000 {
                                Some(val)
                            } else {
                                None
                            }
                        };

                        ctx.store_output(node_id, outputs).await;

                        let _ = channel.send(ExecutionEvent::NodeCompleted {
                            node_id: node_id.clone(),
                            output_preview: preview.clone(),
                            output_data,
                            duration_ms,
                        });

                        node_results.insert(
                            node_id.clone(),
                            NodeResult {
                                success: true,
                                output_preview: Some(preview),
                                error: None,
                                duration_ms,
                            },
                        );
                    }
                    Err(e) => {
                        let error_msg = e.to_string();
                        let _ = channel.send(ExecutionEvent::NodeError {
                            node_id: node_id.clone(),
                            error: error_msg.clone(),
                        });

                        node_results.insert(
                            node_id.clone(),
                            NodeResult {
                                success: false,
                                output_preview: None,
                                error: Some(error_msg.clone()),
                                duration_ms,
                            },
                        );

                        if !error_is_handled_by_try_catch(node_id, doc, &node_map) {
                            had_error = true;
                        }
                    }
                }
            }
        }

        let total_duration_ms = start.elapsed().as_millis() as u64;

        let _ = channel.send(ExecutionEvent::ExecutionComplete { total_duration_ms });

        Ok(ExecutionResult {
            success: !had_error,
            total_duration_ms,
            node_results,
            error: if had_error {
                Some("One or more nodes failed".to_string())
            } else {
                None
            },
        })
    }
}

/// Nodes whose side effects must not be torn mid-flight. File writes run on a
/// blocking thread that keeps going even if the future is dropped, so they
/// finish and cancellation is honored before the next node instead.
fn runs_to_completion(node_type: &str) -> bool {
    node_type == "fileWrite"
}

/// Runs `fut` until it completes or the run is cancelled. Returns `None` on
/// cancellation; the dropped future aborts any in-flight work (e.g. HTTP calls).
async fn run_until_cancelled<F: Future>(fut: F, cancel: &CancelToken) -> Option<F::Output> {
    tokio::select! {
        output = fut => Some(output),
        _ = wait_for_cancel(cancel) => None,
    }
}

async fn wait_for_cancel(cancel: &CancelToken) {
    while !cancel.is_cancelled() {
        tokio::time::sleep(CANCEL_POLL_INTERVAL).await;
    }
}

fn error_is_handled_by_try_catch(
    node_id: &str,
    doc: &FlowDocument,
    node_map: &HashMap<String, &FlowNode>,
) -> bool {
    let mut has_downstream = false;

    for edge in doc.edges.iter().filter(|edge| edge.source == node_id) {
        has_downstream = true;

        let Some(target) = node_map.get(&edge.target) else {
            return false;
        };

        if target.node_type != "tryCatch" {
            return false;
        }
    }

    has_downstream
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{FlowEdge, FlowNode, Position, Viewport};
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::sync::Arc;

    #[test]
    fn try_catch_only_downstream_marks_error_as_handled() {
        let try_node = FlowNode {
            id: "try".into(),
            node_type: "tryCatch".into(),
            position: Position::default(),
            data: serde_json::json!({}),
        };
        let source_node = FlowNode {
            id: "source".into(),
            node_type: "fileRead".into(),
            position: Position::default(),
            data: serde_json::json!({}),
        };
        let doc = FlowDocument {
            id: None,
            name: "Handled".into(),
            nodes: vec![source_node.clone(), try_node.clone()],
            edges: vec![FlowEdge {
                id: "edge-1".into(),
                source: "source".into(),
                target: "try".into(),
                source_handle: None,
                target_handle: None,
            }],
            viewport: Viewport::default(),
        };
        let node_map: HashMap<String, &FlowNode> = doc
            .nodes
            .iter()
            .map(|node| (node.id.clone(), node))
            .collect();

        assert!(error_is_handled_by_try_catch("source", &doc, &node_map));
    }

    struct SlowNode;

    #[async_trait::async_trait]
    impl crate::nodes::NodeExecutor for SlowNode {
        fn node_type(&self) -> &'static str {
            "testSlow"
        }

        async fn execute(
            &self,
            _inputs: HashMap<String, NodeValue>,
            _config: serde_json::Value,
            _ctx: &ExecutionContext,
        ) -> Result<HashMap<String, NodeValue>, AppError> {
            tokio::time::sleep(Duration::from_secs(30)).await;
            Ok(HashMap::new())
        }
    }

    fn slow_flow() -> FlowDocument {
        FlowDocument {
            id: None,
            name: "slow".to_string(),
            nodes: vec![FlowNode {
                id: "slow-1".to_string(),
                node_type: "testSlow".to_string(),
                position: Position::default(),
                data: serde_json::json!({}),
            }],
            edges: vec![],
            viewport: Viewport::default(),
        }
    }

    fn engine_with_slow_node() -> Engine {
        let mut engine = Engine::new();
        engine.registry.register(Box::new(SlowNode));
        engine
    }

    fn silent_channel() -> Channel<ExecutionEvent> {
        Channel::new(|_| Ok(()))
    }

    #[tokio::test]
    async fn stop_interrupts_a_running_node() {
        let generation = Arc::new(AtomicU64::new(0));
        let token = CancelToken::for_run(generation.clone());
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(50)).await;
            generation.fetch_add(1, Ordering::Relaxed);
        });

        let started = Instant::now();
        let result = engine_with_slow_node()
            .execute(&slow_flow(), &silent_channel(), token)
            .await;

        assert!(matches!(result, Err(AppError::Cancelled)));
        assert!(started.elapsed() < Duration::from_secs(5));
    }

    #[tokio::test]
    async fn stop_before_the_run_starts_is_not_lost() {
        let generation = Arc::new(AtomicU64::new(0));
        // execute_flow captures the token before waiting on the engine lock...
        let token = CancelToken::for_run(generation.clone());
        // ...and Stop lands while it waits.
        generation.fetch_add(1, Ordering::Relaxed);

        let result = engine_with_slow_node()
            .execute(&slow_flow(), &silent_channel(), token)
            .await;

        assert!(matches!(result, Err(AppError::Cancelled)));
    }

    #[tokio::test]
    async fn a_new_run_is_not_cancelled_by_an_earlier_stop() {
        let generation = Arc::new(AtomicU64::new(0));
        generation.fetch_add(1, Ordering::Relaxed);
        let token = CancelToken::for_run(generation);
        assert!(!token.is_cancelled());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn stop_interrupts_a_runaway_code_node_as_cancelled() {
        let generation = Arc::new(AtomicU64::new(0));
        let token = CancelToken::for_run(generation.clone());
        // The JS sandbox blocks its worker, so signal Stop from a plain thread.
        std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(100));
            generation.fetch_add(1, Ordering::Relaxed);
        });
        let flow = FlowDocument {
            id: None,
            name: "runaway".to_string(),
            nodes: vec![FlowNode {
                id: "code-1".to_string(),
                node_type: "code".to_string(),
                position: Position::default(),
                data: serde_json::json!({ "code": "while (true) {};" }),
            }],
            edges: vec![],
            viewport: Viewport::default(),
        };

        let started = Instant::now();
        let result = Engine::new().execute(&flow, &silent_channel(), token).await;

        assert!(matches!(result, Err(AppError::Cancelled)));
        assert!(started.elapsed() >= Duration::from_millis(100));
        assert!(started.elapsed() < Duration::from_secs(5));
    }

    #[tokio::test]
    async fn run_until_cancelled_returns_output_when_not_cancelled() {
        let out = run_until_cancelled(async { 42 }, &CancelToken::default()).await;
        assert_eq!(out, Some(42));
    }
}
