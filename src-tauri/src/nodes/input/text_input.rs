use async_trait::async_trait;
use std::collections::HashMap;

use crate::engine::context::ExecutionContext;
use crate::error::AppError;
use crate::nodes::NodeExecutor;
use crate::types::NodeValue;

pub struct TextInputExecutor;

#[async_trait]
impl NodeExecutor for TextInputExecutor {
    fn node_type(&self) -> &'static str {
        "textInput"
    }

    async fn execute(
        &self,
        _inputs: HashMap<String, NodeValue>,
        config: serde_json::Value,
        _ctx: &ExecutionContext,
    ) -> Result<HashMap<String, NodeValue>, AppError> {
        let value = config
            .get("value")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let mut outputs = HashMap::new();
        outputs.insert("value".to_string(), NodeValue::String(value));
        Ok(outputs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_text_input_returns_configured_value() {
        let executor = TextInputExecutor;
        let config = serde_json::json!({ "value": "hello world" });
        let ctx = ExecutionContext::new();
        let result = executor
            .execute(HashMap::new(), config, &ctx)
            .await
            .unwrap();
        assert_eq!(
            result.get("value").unwrap().as_string().unwrap(),
            "hello world"
        );
    }

    #[tokio::test]
    async fn test_text_input_empty_default() {
        let executor = TextInputExecutor;
        let config = serde_json::json!({});
        let ctx = ExecutionContext::new();
        let result = executor
            .execute(HashMap::new(), config, &ctx)
            .await
            .unwrap();
        assert_eq!(result.get("value").unwrap().as_string().unwrap(), "");
    }
}
