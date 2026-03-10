use async_trait::async_trait;
use std::collections::HashMap;

use crate::engine::context::ExecutionContext;
use crate::error::AppError;
use crate::nodes::NodeExecutor;
use crate::types::NodeValue;

pub struct TryCatchExecutor;

#[async_trait]
impl NodeExecutor for TryCatchExecutor {
    fn node_type(&self) -> &'static str {
        "tryCatch"
    }

    async fn execute(
        &self,
        inputs: HashMap<String, NodeValue>,
        _config: serde_json::Value,
        _ctx: &ExecutionContext,
    ) -> Result<HashMap<String, NodeValue>, AppError> {
        let upstream_error = inputs
            .get("__upstreamError")
            .and_then(|value| value.as_string());
        let input = inputs.get("input").cloned().unwrap_or(NodeValue::Null);

        let mut outputs = HashMap::new();
        if let Some(error) = upstream_error {
            outputs.insert("success".to_string(), NodeValue::Null);
            outputs.insert("error".to_string(), NodeValue::String(error));
        } else {
            outputs.insert("success".to_string(), input);
            outputs.insert("error".to_string(), NodeValue::Null);
        }

        Ok(outputs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_try_catch_success() {
        let executor = TryCatchExecutor;
        let mut inputs = HashMap::new();
        inputs.insert("input".to_string(), NodeValue::String("data".into()));

        let ctx = ExecutionContext::new();
        let result = executor
            .execute(inputs, serde_json::json!({}), &ctx)
            .await
            .unwrap();

        assert!(matches!(
            result.get("success").unwrap(),
            NodeValue::String(_)
        ));
        assert!(matches!(result.get("error").unwrap(), NodeValue::Null));
    }

    #[tokio::test]
    async fn test_try_catch_routes_upstream_error() {
        let executor = TryCatchExecutor;
        let mut inputs = HashMap::new();
        inputs.insert(
            "__upstreamError".to_string(),
            NodeValue::String("Node error [fileRead]: file missing".into()),
        );

        let ctx = ExecutionContext::new();
        let result = executor
            .execute(inputs, serde_json::json!({}), &ctx)
            .await
            .unwrap();

        assert!(matches!(result.get("success").unwrap(), NodeValue::Null));
        assert!(matches!(result.get("error").unwrap(), NodeValue::String(_)));
    }
}
