use super::Database;
use crate::error::AppError;
use crate::types::ExecutionResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionRecord {
    pub id: i64,
    pub flow_id: String,
    pub success: bool,
    pub duration_ms: u64,
    pub error: Option<String>,
    pub executed_at: String,
}

impl Database {
    pub fn save_execution(&self, flow_id: &str, result: &ExecutionResult) -> Result<(), AppError> {
        let conn = self.conn()?;
        let result_data = serde_json::to_string(result)?;

        conn.execute(
            "INSERT INTO executions (flow_id, success, duration_ms, result_data, error) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![
                flow_id,
                result.success as i32,
                result.total_duration_ms as i64,
                result_data,
                result.error,
            ],
        )
        .map_err(|e| AppError::Database(format!("Failed to save execution: {}", e)))?;

        Ok(())
    }

    pub fn get_execution_history(&self, flow_id: &str) -> Result<Vec<ExecutionRecord>, AppError> {
        let conn = self.conn()?;
        let mut stmt = conn
            .prepare(
                "SELECT id, flow_id, success, duration_ms, error, executed_at
                 FROM executions WHERE flow_id = ?1 ORDER BY executed_at DESC LIMIT 50",
            )
            .map_err(|e| AppError::Database(e.to_string()))?;

        let records = stmt
            .query_map([flow_id], |row| {
                Ok(ExecutionRecord {
                    id: row.get(0)?,
                    flow_id: row.get(1)?,
                    success: row.get::<_, i32>(2)? != 0,
                    duration_ms: row.get::<_, i64>(3)? as u64,
                    error: row.get(4)?,
                    executed_at: row.get(5)?,
                })
            })
            .map_err(|e| AppError::Database(e.to_string()))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(records)
    }
}
