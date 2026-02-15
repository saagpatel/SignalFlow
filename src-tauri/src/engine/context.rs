use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::error::AppError;
use crate::types::NodeValue;

pub struct ExecutionContext {
    pub node_outputs: Arc<RwLock<HashMap<String, HashMap<String, NodeValue>>>>,
    pub cancelled: Arc<AtomicBool>,
    pub current_node_id: Arc<RwLock<Option<String>>>,
}

impl ExecutionContext {
    pub fn new() -> Self {
        Self {
            node_outputs: Arc::new(RwLock::new(HashMap::new())),
            cancelled: Arc::new(AtomicBool::new(false)),
            current_node_id: Arc::new(RwLock::new(None)),
        }
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Relaxed)
    }

    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Relaxed);
    }

    pub async fn store_output(&self, node_id: &str, outputs: HashMap<String, NodeValue>) {
        let mut lock = self.node_outputs.write().await;
        lock.insert(node_id.to_string(), outputs);
    }

    pub async fn get_input(
        &self,
        source_node_id: &str,
        source_handle: &str,
    ) -> NodeValue {
        let lock = self.node_outputs.read().await;
        lock.get(source_node_id)
            .and_then(|outputs| outputs.get(source_handle))
            .cloned()
            .unwrap_or(NodeValue::Null)
    }

    pub async fn set_current_node_id(&self, node_id: Option<String>) {
        let mut lock = self.current_node_id.write().await;
        *lock = node_id;
    }

    pub async fn get_current_node_id(&self) -> Option<String> {
        let lock = self.current_node_id.read().await;
        lock.clone()
    }

    /// Create a NodeExecution error with the current node ID
    pub async fn error(&self, message: impl Into<String>) -> AppError {
        let node_id = self.get_current_node_id().await.unwrap_or_default();
        AppError::NodeExecution {
            node_id,
            message: message.into(),
        }
    }
}
