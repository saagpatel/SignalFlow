use crate::error::AppError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaStatus {
    pub available: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub name: String,
    pub size: u64,
    pub modified_at: String,
}

pub struct OllamaClient {
    base_url: String,
    client: reqwest::Client,
}

impl OllamaClient {
    pub fn new(base_url: &str) -> Result<Self, AppError> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(120))
            .build()
            .map_err(|e| AppError::Ollama(format!("Failed to create HTTP client: {}", e)))?;
        Ok(Self {
            base_url: base_url.to_string(),
            client,
        })
    }

    pub fn try_default() -> Result<Self, AppError> {
        Self::new("http://localhost:11434")
    }

    pub async fn check_health(&self) -> OllamaStatus {
        match self.client.get(&self.base_url).send().await {
            Ok(resp) if resp.status().is_success() => OllamaStatus {
                available: true,
                error: None,
            },
            Ok(resp) => OllamaStatus {
                available: false,
                error: Some(format!("Ollama returned status {}", resp.status())),
            },
            Err(e) => OllamaStatus {
                available: false,
                error: Some(format!("Cannot connect to Ollama: {}", e)),
            },
        }
    }

    pub async fn list_models(&self) -> Result<Vec<ModelInfo>, AppError> {
        let url = format!("{}/api/tags", self.base_url);
        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| AppError::Ollama(format!("Failed to list models: {}", e)))?;

        let body: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| AppError::Ollama(format!("Invalid response: {}", e)))?;

        let models = body
            .get("models")
            .and_then(|m| m.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|m| {
                        Some(ModelInfo {
                            name: m.get("name")?.as_str()?.to_string(),
                            size: m.get("size")?.as_u64().unwrap_or(0),
                            modified_at: m
                                .get("modified_at")
                                .and_then(|v| v.as_str())
                                .unwrap_or("")
                                .to_string(),
                        })
                    })
                    .collect()
            })
            .unwrap_or_default();

        Ok(models)
    }

    pub async fn generate(
        &self,
        model: &str,
        prompt: &str,
        system: Option<&str>,
        temperature: f64,
    ) -> Result<String, AppError> {
        let url = format!("{}/api/generate", self.base_url);

        let mut body = serde_json::json!({
            "model": model,
            "prompt": prompt,
            "stream": false,
            "options": {
                "temperature": temperature,
            }
        });

        if let Some(sys) = system {
            body["system"] = serde_json::Value::String(sys.to_string());
        }

        let resp = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| AppError::Ollama(format!("Generation failed: {}", e)))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(AppError::Ollama(format!(
                "Ollama returned {}: {}",
                status, text
            )));
        }

        let result: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| AppError::Ollama(format!("Invalid response: {}", e)))?;

        result
            .get("response")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| AppError::Ollama("No response field in output".to_string()))
    }

    pub async fn chat(
        &self,
        model: &str,
        messages: &[serde_json::Value],
        temperature: f64,
    ) -> Result<String, AppError> {
        let url = format!("{}/api/chat", self.base_url);

        let body = serde_json::json!({
            "model": model,
            "messages": messages,
            "stream": false,
            "options": {
                "temperature": temperature,
            }
        });

        let resp = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| AppError::Ollama(format!("Chat failed: {}", e)))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(AppError::Ollama(format!(
                "Ollama returned {}: {}",
                status, text
            )));
        }

        let result: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| AppError::Ollama(format!("Invalid response: {}", e)))?;

        result
            .get("message")
            .and_then(|m| m.get("content"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| AppError::Ollama("No message content in response".to_string()))
    }
}
