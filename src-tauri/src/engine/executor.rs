use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

use tauri::ipc::Channel;
use tokio::sync::RwLock;

use crate::error::AppError;
use crate::nodes::registry::NodeRegistry;
use crate::types::*;

use super::context::ExecutionContext;
use super::graph::FlowGraph;

pub struct Engine {
    registry: NodeRegistry,
    cancel_flag: Arc<std::sync::atomic::AtomicBool>,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            registry: NodeRegistry::new(),
            cancel_flag: Arc::new(std::sync::atomic::AtomicBool::new(false)),
        }
    }

    pub fn cancel(&self) {
        self.cancel_flag
            .store(true, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn reset_cancel(&self) {
        self.cancel_flag
            .store(false, std::sync::atomic::Ordering::Relaxed);
    }

    pub async fn execute(
        &self,
        doc: &FlowDocument,
        channel: &Channel<ExecutionEvent>,
    ) -> Result<ExecutionResult, AppError> {
        self.reset_cancel();
        let start = Instant::now();
        let flow_graph = FlowGraph::from_document(doc)?;

        let mut ctx = ExecutionContext::new();
        // Replace cancel flag with the one from the engine
        ctx.cancelled = self.cancel_flag.clone();

        let node_map: HashMap<String, &FlowNode> =
            doc.nodes.iter().map(|n| (n.id.clone(), n)).collect();

        let mut node_results: HashMap<String, NodeResult> = HashMap::new();
        let mut had_error = false;

        for layer in &flow_graph.execution_layers {
            if ctx.is_cancelled() {
                return Err(AppError::Cancelled);
            }

            for node_id in layer {
                let node = node_map
                    .get(node_id)
                    .ok_or_else(|| AppError::Graph(format!("Node {} not found", node_id)))?;

                let executor = self.registry.get(&node.node_type).ok_or_else(|| {
                    AppError::NodeExecution {
                        node_id: node_id.clone(),
                        message: format!("Unknown node type: {}", node.node_type),
                    }
                })?;

                // Gather inputs from upstream node outputs
                let input_edges = flow_graph.get_input_edges(node_id);
                let mut inputs = HashMap::new();

                for edge in &input_edges {
                    let handle = edge.target_handle.as_deref().unwrap_or("input");
                    let source_handle = edge.source_handle.as_deref().unwrap_or("value");
                    let value = ctx.get_input(&edge.source, source_handle).await;
                    inputs.insert(handle.to_string(), value);
                }

                let _ = channel.send(ExecutionEvent::NodeStarted {
                    node_id: node_id.clone(),
                });

                // Set current node ID for error context
                ctx.set_current_node_id(Some(node_id.clone())).await;

                let node_start = Instant::now();
                let result = executor.execute(inputs, node.data.clone(), &ctx).await;
                let duration_ms = node_start.elapsed().as_millis() as u64;

                // Clear current node ID after execution
                ctx.set_current_node_id(None).await;

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

                        had_error = true;
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
