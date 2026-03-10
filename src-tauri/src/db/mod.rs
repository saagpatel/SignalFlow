pub mod executions;
pub mod flows;
pub mod settings;

use rusqlite::Connection;
use std::path::PathBuf;
use std::sync::Mutex;

use crate::error::AppError;

pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    pub fn open(path: &PathBuf) -> Result<Self, AppError> {
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                AppError::Database(format!("Failed to create database directory: {}", e))
            })?;
        }

        let conn = Connection::open(path)
            .map_err(|e| AppError::Database(format!("Failed to open database: {}", e)))?;

        // Enable WAL mode for better concurrency
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")
            .map_err(|e| AppError::Database(format!("Failed to set pragmas: {}", e)))?;

        let db = Self {
            conn: Mutex::new(conn),
        };

        db.run_migrations()?;

        Ok(db)
    }

    fn run_migrations(&self) -> Result<(), AppError> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| AppError::Database(format!("Failed to lock database: {}", e)))?;

        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS flows (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                data TEXT NOT NULL,
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                updated_at TEXT NOT NULL DEFAULT (datetime('now'))
            );

            CREATE TABLE IF NOT EXISTS executions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                flow_id TEXT NOT NULL,
                success INTEGER NOT NULL,
                duration_ms INTEGER NOT NULL,
                result_data TEXT,
                error TEXT,
                executed_at TEXT NOT NULL DEFAULT (datetime('now')),
                FOREIGN KEY (flow_id) REFERENCES flows(id) ON DELETE CASCADE
            );

            CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );
            ",
        )
        .map_err(|e| AppError::Database(format!("Migration failed: {}", e)))?;

        Ok(())
    }

    pub fn conn(&self) -> Result<std::sync::MutexGuard<'_, Connection>, AppError> {
        self.conn
            .lock()
            .map_err(|e| AppError::Database(format!("Failed to lock database: {}", e)))
    }
}
