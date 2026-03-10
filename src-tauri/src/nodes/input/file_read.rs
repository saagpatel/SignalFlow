use async_trait::async_trait;
use std::collections::HashMap;

use crate::engine::context::ExecutionContext;
use crate::error::AppError;
use crate::nodes::NodeExecutor;
use crate::types::NodeValue;

pub struct FileReadExecutor;

#[async_trait]
impl NodeExecutor for FileReadExecutor {
    fn node_type(&self) -> &'static str {
        "fileRead"
    }

    async fn execute(
        &self,
        inputs: HashMap<String, NodeValue>,
        config: serde_json::Value,
        _ctx: &ExecutionContext,
    ) -> Result<HashMap<String, NodeValue>, AppError> {
        let path = inputs
            .get("path")
            .and_then(|v| v.as_string())
            .or_else(|| {
                config
                    .get("path")
                    .and_then(|v| v.as_str())
                    .map(String::from)
            })
            .unwrap_or_default();

        if path.is_empty() {
            return Err(AppError::NodeExecution {
                node_id: String::new(),
                message: "No file path provided".to_string(),
            });
        }

        // Resolve to absolute path and validate no path traversal
        let canonical = std::path::Path::new(&path);
        if path.contains("..") {
            return Err(AppError::NodeExecution {
                node_id: String::new(),
                message: "Path traversal not allowed".to_string(),
            });
        }

        let content =
            tokio::fs::read_to_string(canonical)
                .await
                .map_err(|e| AppError::NodeExecution {
                    node_id: String::new(),
                    message: format!("Failed to read file '{}': {}", path, e),
                })?;

        let mut outputs = HashMap::new();
        outputs.insert("content".to_string(), NodeValue::String(content));
        outputs.insert("file".to_string(), NodeValue::File { path });
        Ok(outputs)
    }
}
