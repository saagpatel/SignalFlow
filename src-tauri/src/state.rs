use std::sync::Arc;
use tokio::sync::Mutex;

use crate::db::Database;
use crate::engine::Engine;
use crate::error::AppError;

pub const DEFAULT_OLLAMA_ENDPOINT: &str = "http://localhost:11434";

pub struct AppState {
    pub engine: Arc<Mutex<Engine>>,
    pub db: Arc<Database>,
}

impl AppState {
    pub fn new(db_path: std::path::PathBuf) -> Result<Self, AppError> {
        let db = Database::open(&db_path)?;
        Ok(Self {
            engine: Arc::new(Mutex::new(Engine::new())),
            db: Arc::new(db),
        })
    }

    pub fn ollama_endpoint(&self, override_endpoint: Option<String>) -> Result<String, AppError> {
        if let Some(endpoint) = override_endpoint {
            let trimmed = endpoint.trim();
            if !trimmed.is_empty() {
                return Ok(trimmed.to_string());
            }
        }

        Ok(self
            .db
            .get_setting("ollama_endpoint")?
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| DEFAULT_OLLAMA_ENDPOINT.to_string()))
    }
}
