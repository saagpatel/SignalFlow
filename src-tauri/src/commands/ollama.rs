use tauri::State;

use crate::error::AppError;
use crate::ollama::{ModelInfo, OllamaClient, OllamaStatus};
use crate::state::AppState;

#[tauri::command]
pub async fn check_ollama(
    state: State<'_, AppState>,
    endpoint: Option<String>,
) -> Result<OllamaStatus, AppError> {
    let client = OllamaClient::new(&state.ollama_endpoint(endpoint)?)?;
    Ok(client.check_health().await)
}

#[tauri::command]
pub async fn list_models(
    state: State<'_, AppState>,
    endpoint: Option<String>,
) -> Result<Vec<ModelInfo>, AppError> {
    let client = OllamaClient::new(&state.ollama_endpoint(endpoint)?)?;
    client.list_models().await
}
