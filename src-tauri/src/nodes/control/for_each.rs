use async_trait::async_trait;
use std::collections::HashMap;

use crate::engine::context::ExecutionContext;
use crate::error::AppError;
use crate::nodes::NodeExecutor;
use crate::types::NodeValue;

pub struct ForEachExecutor;

#[async_trait]
impl NodeExecutor for ForEachExecutor {
    fn node_type(&self) -> &'static str {
        "forEach"
    }

    async fn execute(
        &self,
        inputs: HashMap<String, NodeValue>,
        _config: serde_json::Value,
        ctx: &ExecutionContext,
    ) -> Result<HashMap<String, NodeValue>, AppError> {
        let array = match inputs.get("array") {
            Some(NodeValue::Array(arr)) => arr.clone(),
            _ => return Err(ctx.error("ForEach expects an array input").await),
        };

        // For now, this node simply passes through the array
        // In a full implementation, this would execute a subgraph for each item
        // and collect the results

        let mut outputs = HashMap::new();
        outputs.insert("results".to_string(), NodeValue::Array(array));

        Ok(outputs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_for_each_passthrough() {
        let executor = ForEachExecutor;
        let mut inputs = HashMap::new();
        inputs.insert(
            "array".to_string(),
            NodeValue::Array(vec![
                NodeValue::Number(1.0),
                NodeValue::Number(2.0),
                NodeValue::Number(3.0),
            ]),
        );

        let ctx = ExecutionContext::new();
        let result = executor
            .execute(inputs, serde_json::json!({}), &ctx)
            .await
            .unwrap();

        match result.get("results").unwrap() {
            NodeValue::Array(arr) => assert_eq!(arr.len(), 3),
            _ => panic!("Expected array output"),
        }
    }
}
