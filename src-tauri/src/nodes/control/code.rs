use async_trait::async_trait;
use std::collections::HashMap;

use crate::engine::context::ExecutionContext;
use crate::error::AppError;
use crate::nodes::NodeExecutor;
use crate::sandbox::evaluate_expression;
use crate::types::NodeValue;

pub struct CodeExecutor;

#[async_trait]
impl NodeExecutor for CodeExecutor {
    fn node_type(&self) -> &'static str {
        "code"
    }

    async fn execute(
        &self,
        inputs: HashMap<String, NodeValue>,
        config: serde_json::Value,
        ctx: &ExecutionContext,
    ) -> Result<HashMap<String, NodeValue>, AppError> {
        // Get input value
        let input = inputs.get("input").cloned().unwrap_or(NodeValue::Null);

        // Get code from config
        let code = match config.get("code").and_then(|v| v.as_str()) {
            Some(c) => c,
            None => return Err(ctx.error("Missing 'code' configuration").await),
        };

        if code.trim().is_empty() {
            return Err(ctx.error("Code is empty").await);
        }

        // Execute the JavaScript code
        let result = match evaluate_expression(code, &input) {
            Ok(r) => r,
            Err(e) => return Err(ctx.error(format!("Code execution failed: {}", e)).await),
        };

        // Return output
        let mut outputs = HashMap::new();
        outputs.insert("output".to_string(), result);
        Ok(outputs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_code_simple_expression() {
        let executor = CodeExecutor;
        let mut inputs = HashMap::new();
        inputs.insert("input".to_string(), NodeValue::Number(5.0));

        let config = serde_json::json!({
            "code": "return input * 2"
        });

        let ctx = ExecutionContext::new();
        let result = executor.execute(inputs, config, &ctx).await.unwrap();

        match result.get("output").unwrap() {
            NodeValue::Number(n) => assert_eq!(*n, 10.0),
            _ => panic!("Expected number output"),
        }
    }

    #[tokio::test]
    async fn test_code_string_manipulation() {
        let executor = CodeExecutor;
        let mut inputs = HashMap::new();
        inputs.insert(
            "input".to_string(),
            NodeValue::String("hello".to_string()),
        );

        let config = serde_json::json!({
            "code": "return input.toUpperCase()"
        });

        let ctx = ExecutionContext::new();
        let result = executor.execute(inputs, config, &ctx).await.unwrap();

        match result.get("output").unwrap() {
            NodeValue::String(s) => assert_eq!(s, "HELLO"),
            _ => panic!("Expected string output"),
        }
    }

    #[tokio::test]
    async fn test_code_array_processing() {
        let executor = CodeExecutor;
        let mut inputs = HashMap::new();
        inputs.insert(
            "input".to_string(),
            NodeValue::Array(vec![
                NodeValue::Number(1.0),
                NodeValue::Number(2.0),
                NodeValue::Number(3.0),
            ]),
        );

        let config = serde_json::json!({
            "code": "return input.map(x => x * 2)"
        });

        let ctx = ExecutionContext::new();
        let result = executor.execute(inputs, config, &ctx).await.unwrap();

        match result.get("output").unwrap() {
            NodeValue::Array(arr) => {
                assert_eq!(arr.len(), 3);
                match &arr[0] {
                    NodeValue::Number(n) => assert_eq!(*n, 2.0),
                    _ => panic!("Expected number in array"),
                }
            }
            _ => panic!("Expected array output"),
        }
    }

    #[tokio::test]
    async fn test_code_missing_config() {
        let executor = CodeExecutor;
        let mut inputs = HashMap::new();
        inputs.insert("input".to_string(), NodeValue::Number(5.0));

        let config = serde_json::json!({});

        let ctx = ExecutionContext::new();
        let result = executor.execute(inputs, config, &ctx).await;

        assert!(result.is_err());
        if let Err(AppError::NodeExecution { message, .. }) = result {
            assert!(message.contains("Missing 'code' configuration"));
        }
    }

    #[tokio::test]
    async fn test_code_empty_code() {
        let executor = CodeExecutor;
        let mut inputs = HashMap::new();
        inputs.insert("input".to_string(), NodeValue::Number(5.0));

        let config = serde_json::json!({
            "code": ""
        });

        let ctx = ExecutionContext::new();
        let result = executor.execute(inputs, config, &ctx).await;

        assert!(result.is_err());
        if let Err(AppError::NodeExecution { message, .. }) = result {
            assert!(message.contains("Code is empty"));
        }
    }

    #[tokio::test]
    async fn test_code_invalid_syntax() {
        let executor = CodeExecutor;
        let mut inputs = HashMap::new();
        inputs.insert("input".to_string(), NodeValue::Number(5.0));

        let config = serde_json::json!({
            "code": "return input +"
        });

        let ctx = ExecutionContext::new();
        let result = executor.execute(inputs, config, &ctx).await;

        assert!(result.is_err());
        if let Err(AppError::NodeExecution { message, .. }) = result {
            assert!(message.contains("Code execution failed"));
        }
    }
}
