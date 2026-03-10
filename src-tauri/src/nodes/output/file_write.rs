use async_trait::async_trait;
use std::collections::HashMap;

use crate::engine::context::ExecutionContext;
use crate::error::AppError;
use crate::nodes::NodeExecutor;
use crate::types::NodeValue;

pub struct FileWriteExecutor;

#[async_trait]
impl NodeExecutor for FileWriteExecutor {
    fn node_type(&self) -> &'static str {
        "fileWrite"
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

        // Validate no path traversal
        if path.contains("..") {
            return Err(AppError::NodeExecution {
                node_id: String::new(),
                message: "Path traversal not allowed".to_string(),
            });
        }

        let content = inputs
            .get("content")
            .map(|v| v.coerce_to_string())
            .unwrap_or_default();

        let append = config
            .get("append")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        if append {
            use tokio::io::AsyncWriteExt;
            let mut file = tokio::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&path)
                .await
                .map_err(|e| AppError::NodeExecution {
                    node_id: String::new(),
                    message: format!("Failed to open file '{}': {}", path, e),
                })?;
            file.write_all(content.as_bytes())
                .await
                .map_err(|e| AppError::Io(e.to_string()))?;
            file.flush()
                .await
                .map_err(|e| AppError::Io(e.to_string()))?;
        } else {
            tokio::fs::write(&path, &content)
                .await
                .map_err(|e| AppError::NodeExecution {
                    node_id: String::new(),
                    message: format!("Failed to write file '{}': {}", path, e),
                })?;
        }

        let mut outputs = HashMap::new();
        outputs.insert("file".to_string(), NodeValue::File { path });
        Ok(outputs)
    }
}
