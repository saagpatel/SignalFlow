use tauri::State;

use crate::db::executions::ExecutionRecord;
use crate::db::flows::FlowSummary;
use crate::error::AppError;
use crate::state::AppState;
use crate::types::*;

#[tauri::command]
pub async fn save_flow(state: State<'_, AppState>, flow: FlowDocument) -> Result<String, AppError> {
    let id = state.db.save_flow(&flow)?;
    Ok(id)
}

#[tauri::command]
pub async fn load_flow(state: State<'_, AppState>, id: String) -> Result<FlowDocument, AppError> {
    state.db.load_flow(&id)
}

#[tauri::command]
pub async fn list_flows(state: State<'_, AppState>) -> Result<Vec<FlowSummary>, AppError> {
    state.db.list_flows()
}

#[tauri::command]
pub async fn delete_flow(state: State<'_, AppState>, id: String) -> Result<(), AppError> {
    state.db.delete_flow(&id)
}

#[tauri::command]
pub async fn get_execution_history(
    state: State<'_, AppState>,
    flow_id: String,
) -> Result<Vec<ExecutionRecord>, AppError> {
    state.db.get_execution_history(&flow_id)
}
