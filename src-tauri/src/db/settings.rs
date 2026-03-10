use super::Database;
use crate::error::AppError;

impl Database {
    pub fn get_setting(&self, key: &str) -> Result<Option<String>, AppError> {
        let conn = self.conn()?;
        let result = conn.query_row("SELECT value FROM settings WHERE key = ?1", [key], |row| {
            row.get(0)
        });

        match result {
            Ok(value) => Ok(Some(value)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(AppError::Database(e.to_string())),
        }
    }

    pub fn set_setting(&self, key: &str, value: &str) -> Result<(), AppError> {
        let conn = self.conn()?;
        conn.execute(
            "INSERT INTO settings (key, value) VALUES (?1, ?2)
             ON CONFLICT(key) DO UPDATE SET value = ?2",
            rusqlite::params![key, value],
        )
        .map_err(|e| AppError::Database(format!("Failed to set setting: {}", e)))?;
        Ok(())
    }
}
