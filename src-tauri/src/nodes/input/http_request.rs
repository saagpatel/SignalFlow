use async_trait::async_trait;
use std::collections::HashMap;

use crate::engine::context::ExecutionContext;
use crate::error::AppError;
use crate::nodes::NodeExecutor;
use crate::types::NodeValue;

pub struct HttpRequestExecutor;

#[async_trait]
impl NodeExecutor for HttpRequestExecutor {
    fn node_type(&self) -> &'static str {
        "httpRequest"
    }

    async fn execute(
        &self,
        inputs: HashMap<String, NodeValue>,
        config: serde_json::Value,
        _ctx: &ExecutionContext,
    ) -> Result<HashMap<String, NodeValue>, AppError> {
        let url = inputs
            .get("url")
            .and_then(|v| v.as_string())
            .or_else(|| config.get("url").and_then(|v| v.as_str()).map(String::from))
            .unwrap_or_default();

        if url.is_empty() {
            return Err(AppError::NodeExecution {
                node_id: String::new(),
                message: "No URL provided".to_string(),
            });
        }

        // Basic URL validation
        if !url.starts_with("http://") && !url.starts_with("https://") {
            return Err(AppError::NodeExecution {
                node_id: String::new(),
                message: "URL must start with http:// or https://".to_string(),
            });
        }

        let method = config
            .get("method")
            .and_then(|v| v.as_str())
            .unwrap_or("GET");
        let timeout_ms = config
            .get("timeoutMs")
            .and_then(|v| v.as_u64())
            .unwrap_or(30_000);

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_millis(timeout_ms))
            .build()
            .map_err(|e| AppError::Http(e.to_string()))?;

        let mut req = match method.to_uppercase().as_str() {
            "POST" => client.post(&url),
            "PUT" => client.put(&url),
            "DELETE" => client.delete(&url),
            "PATCH" => client.patch(&url),
            _ => client.get(&url),
        };

        // Add headers from config
        if let Some(headers_str) = config.get("headers").and_then(|v| v.as_str()) {
            if let Ok(headers) = serde_json::from_str::<HashMap<String, String>>(headers_str) {
                for (k, v) in headers {
                    req = req.header(&k, &v);
                }
            }
        }

        // Add body
        if let Some(body) = inputs.get("body").and_then(|v| v.as_string()) {
            req = req.body(body);
        }

        let response = req.send().await.map_err(|e| AppError::Http(e.to_string()))?;
        let status = response.status().as_u16();
        let body = response
            .text()
            .await
            .map_err(|e| AppError::Http(e.to_string()))?;

        let mut outputs = HashMap::new();
        outputs.insert("response".to_string(), NodeValue::String(body));
        outputs.insert("status".to_string(), NodeValue::Number(status as f64));
        Ok(outputs)
    }
}
