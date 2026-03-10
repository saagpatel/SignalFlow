use async_trait::async_trait;
use std::collections::HashMap;

use crate::engine::context::ExecutionContext;
use crate::error::AppError;
use crate::nodes::NodeExecutor;
use crate::sandbox::evaluate_expression_with_scope;
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
        config: serde_json::Value,
        ctx: &ExecutionContext,
    ) -> Result<HashMap<String, NodeValue>, AppError> {
        let array = match inputs.get("array") {
            Some(NodeValue::Array(arr)) => arr.clone(),
            _ => return Err(ctx.error("ForEach expects an array input").await),
        };

        let expression = config
            .get("expression")
            .and_then(|value| value.as_str())
            .unwrap_or("item");

        let mut results = Vec::with_capacity(array.len());
        for (index, item) in array.into_iter().enumerate() {
            let mut scope = HashMap::new();
            scope.insert("item".to_string(), item);
            scope.insert("index".to_string(), NodeValue::Number(index as f64));

            match evaluate_expression_with_scope(expression, scope) {
                Ok(result) => results.push(result),
                Err(error) => {
                    return Err(ctx
                        .error(format!("ForEach expression error: {error}"))
                        .await)
                }
            }
        }

        let mut outputs = HashMap::new();
        outputs.insert("results".to_string(), NodeValue::Array(results));

        Ok(outputs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_for_each_expression() {
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
            .execute(
                inputs,
                serde_json::json!({ "expression": "item * 2" }),
                &ctx,
            )
            .await
            .unwrap();

        match result.get("results").unwrap() {
            NodeValue::Array(arr) => {
                assert_eq!(arr.len(), 3);
                assert!(matches!(arr[0], NodeValue::Number(2.0)));
                assert!(matches!(arr[1], NodeValue::Number(4.0)));
                assert!(matches!(arr[2], NodeValue::Number(6.0)));
            }
            _ => panic!("Expected array output"),
        }
    }
}
