use async_trait::async_trait;
use std::collections::HashMap;

use crate::engine::context::ExecutionContext;
use crate::error::AppError;
use crate::nodes::NodeExecutor;
use crate::types::NodeValue;

pub struct SplitExecutor;

#[async_trait]
impl NodeExecutor for SplitExecutor {
    fn node_type(&self) -> &'static str {
        "split"
    }

    async fn execute(
        &self,
        inputs: HashMap<String, NodeValue>,
        config: serde_json::Value,
        _ctx: &ExecutionContext,
    ) -> Result<HashMap<String, NodeValue>, AppError> {
        let input = inputs
            .get("input")
            .and_then(|v| v.as_string())
            .unwrap_or_default();

        let delimiter = config
            .get("delimiter")
            .and_then(|v| v.as_str())
            .unwrap_or(",");

        let parts: Vec<NodeValue> = input
            .split(delimiter)
            .map(|s| NodeValue::String(s.trim().to_string()))
            .collect();

        let mut outputs = HashMap::new();
        outputs.insert("output".to_string(), NodeValue::Array(parts));
        Ok(outputs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_split_comma() {
        let executor = SplitExecutor;
        let mut inputs = HashMap::new();
        inputs.insert(
            "input".to_string(),
            NodeValue::String("a, b, c".to_string()),
        );
        let config = serde_json::json!({ "delimiter": "," });
        let ctx = ExecutionContext::new();
        let result = executor.execute(inputs, config, &ctx).await.unwrap();
        if let NodeValue::Array(arr) = result.get("output").unwrap() {
            assert_eq!(arr.len(), 3);
        } else {
            panic!("Expected array");
        }
    }
}
