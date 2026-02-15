use async_trait::async_trait;
use std::collections::HashMap;

use crate::engine::context::ExecutionContext;
use crate::error::AppError;
use crate::nodes::NodeExecutor;
use crate::sandbox::evaluate_expression_with_scope;
use crate::types::NodeValue;

pub struct ConditionalExecutor;

#[async_trait]
impl NodeExecutor for ConditionalExecutor {
    fn node_type(&self) -> &'static str {
        "conditional"
    }

    async fn execute(
        &self,
        inputs: HashMap<String, NodeValue>,
        config: serde_json::Value,
        ctx: &ExecutionContext,
    ) -> Result<HashMap<String, NodeValue>, AppError> {
        let input = inputs.get("input").cloned().unwrap_or(NodeValue::Null);

        // Check for condition input first, then expression config, then default truthy check
        let condition = if let Some(NodeValue::Boolean(b)) = inputs.get("condition") {
            *b
        } else if let Some(expression) = config.get("expression").and_then(|v| v.as_str()) {
            // Evaluate expression
            let mut scope = HashMap::new();
            scope.insert("input".to_string(), input.clone());

            match evaluate_expression_with_scope(expression, scope) {
                Ok(NodeValue::Boolean(b)) => b,
                Ok(other) => {
                    // Try to coerce to boolean
                    other.as_bool().unwrap_or(false)
                }
                Err(e) => {
                    return Err(ctx.error(format!("Conditional expression error: {}", e)).await);
                }
            }
        } else {
            // Default: truthy check on input
            input.as_bool().unwrap_or(false)
        };

        let mut outputs = HashMap::new();
        if condition {
            outputs.insert("true".to_string(), input);
            outputs.insert("false".to_string(), NodeValue::Null);
        } else {
            outputs.insert("true".to_string(), NodeValue::Null);
            outputs.insert("false".to_string(), input);
        }
        Ok(outputs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_conditional_true_branch() {
        let executor = ConditionalExecutor;
        let mut inputs = HashMap::new();
        inputs.insert("input".to_string(), NodeValue::String("data".into()));
        inputs.insert("condition".to_string(), NodeValue::Boolean(true));

        let ctx = ExecutionContext::new();
        let result = executor
            .execute(inputs, serde_json::json!({}), &ctx)
            .await
            .unwrap();
        assert!(matches!(result.get("true").unwrap(), NodeValue::String(_)));
        assert!(matches!(result.get("false").unwrap(), NodeValue::Null));
    }

    #[tokio::test]
    async fn test_conditional_false_branch() {
        let executor = ConditionalExecutor;
        let mut inputs = HashMap::new();
        inputs.insert("input".to_string(), NodeValue::String("data".into()));
        inputs.insert("condition".to_string(), NodeValue::Boolean(false));

        let ctx = ExecutionContext::new();
        let result = executor
            .execute(inputs, serde_json::json!({}), &ctx)
            .await
            .unwrap();
        assert!(matches!(result.get("true").unwrap(), NodeValue::Null));
        assert!(matches!(result.get("false").unwrap(), NodeValue::String(_)));
    }
}
