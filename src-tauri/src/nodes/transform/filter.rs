use async_trait::async_trait;
use std::collections::HashMap;

use crate::engine::context::ExecutionContext;
use crate::error::AppError;
use crate::nodes::NodeExecutor;
use crate::sandbox::evaluate_expression_with_scope;
use crate::types::NodeValue;

pub struct FilterExecutor;

#[async_trait]
impl NodeExecutor for FilterExecutor {
    fn node_type(&self) -> &'static str {
        "filter"
    }

    async fn execute(
        &self,
        inputs: HashMap<String, NodeValue>,
        config: serde_json::Value,
        ctx: &ExecutionContext,
    ) -> Result<HashMap<String, NodeValue>, AppError> {
        let input = match inputs.get("input") {
            Some(NodeValue::Array(arr)) => arr.clone(),
            _ => return Err(ctx.error("Filter expects an array input").await),
        };

        let condition = config
            .get("condition")
            .and_then(|v| v.as_str())
            .unwrap_or("item !== null");

        // Evaluate condition for each item
        let mut filtered = Vec::new();
        for (index, item) in input.into_iter().enumerate() {
            let mut scope = HashMap::new();
            scope.insert("item".to_string(), item.clone());
            scope.insert("index".to_string(), NodeValue::Number(index as f64));

            match evaluate_expression_with_scope(condition, scope) {
                Ok(NodeValue::Boolean(true)) => {
                    filtered.push(item);
                }
                Ok(NodeValue::Boolean(false)) => {
                    // Skip this item
                }
                Ok(_) => {
                    // Truthy/falsy check: only keep if explicitly true
                    // For now, only accept boolean results
                }
                Err(e) => {
                    return Err(ctx.error(format!("Filter condition error: {}", e)).await);
                }
            }
        }

        let mut outputs = HashMap::new();
        outputs.insert("output".to_string(), NodeValue::Array(filtered));
        Ok(outputs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_filter_removes_nulls() {
        let executor = FilterExecutor;
        let mut inputs = HashMap::new();
        inputs.insert(
            "input".to_string(),
            NodeValue::Array(vec![
                NodeValue::String("a".into()),
                NodeValue::Null,
                NodeValue::String("b".into()),
            ]),
        );
        let ctx = ExecutionContext::new();
        let result = executor
            .execute(inputs, serde_json::json!({}), &ctx)
            .await
            .unwrap();
        if let NodeValue::Array(arr) = result.get("output").unwrap() {
            assert_eq!(arr.len(), 2);
        } else {
            panic!("Expected array output");
        }
    }
}
