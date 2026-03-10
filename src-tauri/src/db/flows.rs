use super::Database;
use crate::error::AppError;
use crate::types::FlowDocument;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowSummary {
    pub id: String,
    pub name: String,
    pub updated_at: String,
}

impl Database {
    pub fn save_flow(&self, flow: &FlowDocument) -> Result<String, AppError> {
        let conn = self.conn()?;
        let id = flow.id.clone().unwrap_or_else(|| {
            format!(
                "flow_{}",
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_millis()
            )
        });

        let data = serde_json::to_string(flow)?;

        conn.execute(
            "INSERT INTO flows (id, name, data, updated_at) VALUES (?1, ?2, ?3, datetime('now'))
             ON CONFLICT(id) DO UPDATE SET name = ?2, data = ?3, updated_at = datetime('now')",
            rusqlite::params![id, flow.name, data],
        )
        .map_err(|e| AppError::Database(format!("Failed to save flow: {}", e)))?;

        Ok(id)
    }

    pub fn load_flow(&self, id: &str) -> Result<FlowDocument, AppError> {
        let conn = self.conn()?;
        let data: String = conn
            .query_row("SELECT data FROM flows WHERE id = ?1", [id], |row| {
                row.get(0)
            })
            .map_err(|e| AppError::Database(format!("Flow not found: {}", e)))?;

        let mut flow: FlowDocument = serde_json::from_str(&data)?;
        flow.id = Some(id.to_string());
        Ok(flow)
    }

    pub fn list_flows(&self) -> Result<Vec<FlowSummary>, AppError> {
        let conn = self.conn()?;
        let mut stmt = conn
            .prepare("SELECT id, name, updated_at FROM flows ORDER BY updated_at DESC")
            .map_err(|e| AppError::Database(e.to_string()))?;

        let flows = stmt
            .query_map([], |row| {
                Ok(FlowSummary {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    updated_at: row.get(2)?,
                })
            })
            .map_err(|e| AppError::Database(e.to_string()))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(flows)
    }

    pub fn delete_flow(&self, id: &str) -> Result<(), AppError> {
        let conn = self.conn()?;
        conn.execute("DELETE FROM flows WHERE id = ?1", [id])
            .map_err(|e| AppError::Database(format!("Failed to delete flow: {}", e)))?;
        Ok(())
    }
}
