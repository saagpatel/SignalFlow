use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Node error [{node_id}]: {message}")]
    NodeExecution { node_id: String, message: String },

    #[error("Graph error: {0}")]
    Graph(String),

    #[error("Cycle detected in graph")]
    CycleDetected,

    #[error("Execution cancelled")]
    Cancelled,

    #[error("Database error: {0}")]
    Database(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("IO error: {0}")]
    Io(String),

    #[error("HTTP error: {0}")]
    Http(String),

    #[error("Ollama error: {0}")]
    Ollama(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("{0}")]
    Other(String),
}

impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> Self {
        AppError::Serialization(e.to_string())
    }
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::Io(e.to_string())
    }
}

impl Serialize for AppError {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}
