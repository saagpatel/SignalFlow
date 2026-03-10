use async_trait::async_trait;
use std::collections::HashMap;

use crate::engine::context::ExecutionContext;
use crate::error::AppError;
use crate::nodes::NodeExecutor;
use crate::types::NodeValue;

pub struct NumberInputExecutor;

#[async_trait]
impl NodeExecutor for NumberInputExecutor {
    fn node_type(&self) -> &'static str {
        "numberInput"
    }

    async fn execute(
        &self,
        _inputs: HashMap<String, NodeValue>,
        config: serde_json::Value,
        _ctx: &ExecutionContext,
    ) -> Result<HashMap<String, NodeValue>, AppError> {
        let value = config.get("value").and_then(|v| v.as_f64()).unwrap_or(0.0);

        let mut outputs = HashMap::new();
        outputs.insert("value".to_string(), NodeValue::Number(value));
        Ok(outputs)
    }
}
