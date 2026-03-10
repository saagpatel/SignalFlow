use async_trait::async_trait;
use std::collections::HashMap;

use crate::engine::context::ExecutionContext;
use crate::error::AppError;
use crate::nodes::NodeExecutor;
use crate::types::NodeValue;

pub struct RegexExecutor;

#[async_trait]
impl NodeExecutor for RegexExecutor {
    fn node_type(&self) -> &'static str {
        "regex"
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

        let pattern = config.get("pattern").and_then(|v| v.as_str()).unwrap_or("");

        let mode = config
            .get("mode")
            .and_then(|v| v.as_str())
            .unwrap_or("match");

        if pattern.len() > 1000 {
            return Err(AppError::NodeExecution {
                node_id: String::new(),
                message: "Regex pattern too long (max 1000 chars)".to_string(),
            });
        }

        let re = regex::Regex::new(pattern).map_err(|e| AppError::NodeExecution {
            node_id: String::new(),
            message: format!("Invalid regex: {}", e),
        })?;

        let mut outputs = HashMap::new();

        match mode {
            "replace" => {
                let replacement = config
                    .get("replacement")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let result = re.replace_all(&input, replacement).to_string();
                outputs.insert("result".to_string(), NodeValue::String(result));
                outputs.insert("matches".to_string(), NodeValue::Array(vec![]));
            }
            _ => {
                // match mode
                let matches: Vec<NodeValue> = re
                    .find_iter(&input)
                    .map(|m| NodeValue::String(m.as_str().to_string()))
                    .collect();
                outputs.insert("matches".to_string(), NodeValue::Array(matches));
                outputs.insert("result".to_string(), NodeValue::String(input));
            }
        }

        Ok(outputs)
    }
}
