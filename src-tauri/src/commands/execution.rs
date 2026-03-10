use tauri::ipc::Channel;
use tauri::State;

use crate::error::AppError;
use crate::state::AppState;
use crate::types::*;

#[tauri::command]
pub async fn execute_flow(
    state: State<'_, AppState>,
    flow: FlowDocument,
    on_progress: Channel<ExecutionEvent>,
) -> Result<ExecutionResult, AppError> {
    let mut flow = flow;
    inject_ollama_endpoint(&mut flow, &state.ollama_endpoint(None)?);

    let engine = state.engine.lock().await;
    let result = engine.execute(&flow, &on_progress).await?;

    // Save execution history — log failures but don't block the response
    if let Some(ref flow_id) = flow.id {
        if let Err(e) = state.db.save_execution(flow_id, &result) {
            eprintln!("Failed to save execution history: {}", e);
        }
    }

    Ok(result)
}

#[tauri::command]
pub async fn stop_execution(state: State<'_, AppState>) -> Result<(), AppError> {
    let engine = state.engine.lock().await;
    engine.cancel();
    Ok(())
}

fn inject_ollama_endpoint(flow: &mut FlowDocument, endpoint: &str) {
    for node in &mut flow.nodes {
        if node.node_type != "llmPrompt" && node.node_type != "llmChat" {
            continue;
        }

        let object = match node.data.as_object_mut() {
            Some(object) => object,
            None => {
                node.data = serde_json::json!({});
                node.data
                    .as_object_mut()
                    .expect("empty JSON object should be mutable")
            }
        };

        object.insert(
            "endpoint".to_string(),
            serde_json::Value::String(endpoint.to_string()),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{FlowEdge, FlowNode, Position, Viewport};

    #[test]
    fn inject_ollama_endpoint_only_updates_ai_nodes() {
        let mut flow = FlowDocument {
            id: None,
            name: "Test".to_string(),
            nodes: vec![
                FlowNode {
                    id: "ai-1".to_string(),
                    node_type: "llmPrompt".to_string(),
                    position: Position::default(),
                    data: serde_json::json!({ "model": "llama3.2" }),
                },
                FlowNode {
                    id: "tx-1".to_string(),
                    node_type: "textInput".to_string(),
                    position: Position::default(),
                    data: serde_json::json!({ "value": "hello" }),
                },
            ],
            edges: Vec::<FlowEdge>::new(),
            viewport: Viewport::default(),
        };

        inject_ollama_endpoint(&mut flow, "http://example.com");

        assert_eq!(
            flow.nodes[0]
                .data
                .get("endpoint")
                .and_then(|value| value.as_str()),
            Some("http://example.com")
        );
        assert_eq!(flow.nodes[1].data.get("endpoint"), None);
    }
}
