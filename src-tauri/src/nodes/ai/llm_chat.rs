use async_trait::async_trait;
use std::collections::HashMap;

use crate::engine::context::ExecutionContext;
use crate::error::AppError;
use crate::nodes::NodeExecutor;
use crate::ollama::OllamaClient;
use crate::state::DEFAULT_OLLAMA_ENDPOINT;
use crate::types::NodeValue;

pub struct LlmChatExecutor;

#[async_trait]
impl NodeExecutor for LlmChatExecutor {
    fn node_type(&self) -> &'static str {
        "llmChat"
    }

    async fn execute(
        &self,
        inputs: HashMap<String, NodeValue>,
        config: serde_json::Value,
        _ctx: &ExecutionContext,
    ) -> Result<HashMap<String, NodeValue>, AppError> {
        let message = inputs
            .get("message")
            .and_then(|v| v.as_string())
            .ok_or_else(|| AppError::NodeExecution {
                node_id: String::new(),
                message: "No message provided".to_string(),
            })?;

        let model = config
            .get("model")
            .and_then(|v| v.as_str())
            .unwrap_or("llama3.2");

        let temperature = config
            .get("temperature")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.7);

        let system_prompt = config
            .get("systemPrompt")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        // Build messages array
        let mut messages: Vec<serde_json::Value> = vec![];

        if !system_prompt.is_empty() {
            messages.push(serde_json::json!({
                "role": "system",
                "content": system_prompt
            }));
        }

        // Append history if provided
        if let Some(NodeValue::Array(history)) = inputs.get("history") {
            for item in history {
                if let NodeValue::Object(obj) = item {
                    if let (Some(role), Some(content)) = (obj.get("role"), obj.get("content")) {
                        messages.push(serde_json::json!({
                            "role": role,
                            "content": content
                        }));
                    }
                }
            }
        }

        messages.push(serde_json::json!({
            "role": "user",
            "content": message
        }));

        let endpoint = config
            .get("endpoint")
            .and_then(|value| value.as_str())
            .unwrap_or(DEFAULT_OLLAMA_ENDPOINT);

        let client = OllamaClient::new(endpoint)?;
        let response = client.chat(model, &messages, temperature).await?;

        // Add assistant response to history
        messages.push(serde_json::json!({
            "role": "assistant",
            "content": response
        }));

        let history_out: Vec<NodeValue> = messages
            .iter()
            .map(|m| {
                let mut obj = HashMap::new();
                obj.insert(
                    "role".to_string(),
                    m.get("role").cloned().unwrap_or_default(),
                );
                obj.insert(
                    "content".to_string(),
                    m.get("content").cloned().unwrap_or_default(),
                );
                NodeValue::Object(obj)
            })
            .collect();

        let mut outputs = HashMap::new();
        outputs.insert("response".to_string(), NodeValue::String(response));
        outputs.insert("history".to_string(), NodeValue::Array(history_out));
        Ok(outputs)
    }
}
