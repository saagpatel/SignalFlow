use async_trait::async_trait;
use std::collections::HashMap;

use crate::engine::context::ExecutionContext;
use crate::error::AppError;
use crate::nodes::NodeExecutor;
use crate::sandbox::evaluate_expression_with_scope;
use crate::types::NodeValue;

pub struct MapExecutor;

#[async_trait]
impl NodeExecutor for MapExecutor {
    fn node_type(&self) -> &'static str {
        "map"
    }

    async fn execute(
        &self,
        inputs: HashMap<String, NodeValue>,
        config: serde_json::Value,
        ctx: &ExecutionContext,
    ) -> Result<HashMap<String, NodeValue>, AppError> {
        let input = match inputs.get("input") {
            Some(NodeValue::Array(arr)) => arr.clone(),
            _ => return Err(ctx.error("Map expects an array input").await),
        };

        let expression = config
            .get("expression")
            .and_then(|v| v.as_str())
            .unwrap_or("item");

        // Evaluate expression for each item
        let mut mapped = Vec::new();
        for (index, item) in input.into_iter().enumerate() {
            let mut scope = HashMap::new();
            scope.insert("item".to_string(), item.clone());
            scope.insert("index".to_string(), NodeValue::Number(index as f64));

            match evaluate_expression_with_scope(expression, scope) {
                Ok(result) => {
                    mapped.push(result);
                }
                Err(e) => {
                    return Err(ctx.error(format!("Map expression error: {}", e)).await);
                }
            }
        }

        let mut outputs = HashMap::new();
        outputs.insert("output".to_string(), NodeValue::Array(mapped));
        Ok(outputs)
    }
}
