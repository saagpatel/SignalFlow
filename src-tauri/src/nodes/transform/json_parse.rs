use async_trait::async_trait;
use std::collections::HashMap;

use crate::engine::context::ExecutionContext;
use crate::error::AppError;
use crate::nodes::NodeExecutor;
use crate::types::NodeValue;

pub struct JsonParseExecutor;

#[async_trait]
impl NodeExecutor for JsonParseExecutor {
    fn node_type(&self) -> &'static str {
        "jsonParse"
    }

    async fn execute(
        &self,
        inputs: HashMap<String, NodeValue>,
        _config: serde_json::Value,
        _ctx: &ExecutionContext,
    ) -> Result<HashMap<String, NodeValue>, AppError> {
        let input_str = inputs
            .get("input")
            .and_then(|v| v.as_string())
            .ok_or_else(|| AppError::NodeExecution {
                node_id: String::new(),
                message: "No input provided to JSON Parse".to_string(),
            })?;

        let parsed: serde_json::Value =
            serde_json::from_str(&input_str).map_err(|e| AppError::NodeExecution {
                node_id: String::new(),
                message: format!("Invalid JSON: {}", e),
            })?;

        let output = match parsed {
            serde_json::Value::Object(map) => NodeValue::Object(map.into_iter().collect()),
            serde_json::Value::Array(arr) => {
                NodeValue::Array(arr.into_iter().map(json_to_node_value).collect())
            }
            other => json_to_node_value(other),
        };

        let mut outputs = HashMap::new();
        outputs.insert("output".to_string(), output);
        Ok(outputs)
    }
}

fn json_to_node_value(v: serde_json::Value) -> NodeValue {
    match v {
        serde_json::Value::Null => NodeValue::Null,
        serde_json::Value::Bool(b) => NodeValue::Boolean(b),
        serde_json::Value::Number(n) => NodeValue::Number(n.as_f64().unwrap_or(0.0)),
        serde_json::Value::String(s) => NodeValue::String(s),
        serde_json::Value::Array(arr) => {
            NodeValue::Array(arr.into_iter().map(json_to_node_value).collect())
        }
        serde_json::Value::Object(map) => NodeValue::Object(map.into_iter().collect()),
    }
}
