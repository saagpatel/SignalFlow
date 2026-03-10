use async_trait::async_trait;
use std::collections::HashMap;

use crate::engine::context::ExecutionContext;
use crate::error::AppError;
use crate::nodes::NodeExecutor;
use crate::ollama::OllamaClient;
use crate::state::DEFAULT_OLLAMA_ENDPOINT;
use crate::types::NodeValue;

pub struct LlmPromptExecutor;

#[async_trait]
impl NodeExecutor for LlmPromptExecutor {
    fn node_type(&self) -> &'static str {
        "llmPrompt"
    }

    async fn execute(
        &self,
        inputs: HashMap<String, NodeValue>,
        config: serde_json::Value,
        _ctx: &ExecutionContext,
    ) -> Result<HashMap<String, NodeValue>, AppError> {
        let prompt = inputs
            .get("prompt")
            .and_then(|v| v.as_string())
            .ok_or_else(|| AppError::NodeExecution {
                node_id: String::new(),
                message: "No prompt provided".to_string(),
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
            .filter(|s| !s.is_empty());

        let endpoint = config
            .get("endpoint")
            .and_then(|value| value.as_str())
            .unwrap_or(DEFAULT_OLLAMA_ENDPOINT);

        let client = OllamaClient::new(endpoint)?;
        let response = client
            .generate(model, &prompt, system_prompt, temperature)
            .await?;

        let mut outputs = HashMap::new();
        outputs.insert("response".to_string(), NodeValue::String(response));
        Ok(outputs)
    }
}
