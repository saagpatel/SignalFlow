use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowDocument {
    pub id: Option<String>,
    pub name: String,
    pub nodes: Vec<FlowNode>,
    pub edges: Vec<FlowEdge>,
    pub viewport: Viewport,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowNode {
    pub id: String,
    #[serde(rename = "type")]
    pub node_type: String,
    pub position: Position,
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowEdge {
    pub id: String,
    pub source: String,
    pub target: String,
    #[serde(rename = "sourceHandle")]
    pub source_handle: Option<String>,
    #[serde(rename = "targetHandle")]
    pub target_handle: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Viewport {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(untagged)]
pub enum NodeValue {
    #[default]
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<NodeValue>),
    Object(HashMap<String, serde_json::Value>),
    File {
        path: String,
    },
}

impl NodeValue {
    pub fn as_string(&self) -> Option<String> {
        match self {
            NodeValue::String(s) => Some(s.clone()),
            NodeValue::Number(n) => Some(n.to_string()),
            NodeValue::Boolean(b) => Some(b.to_string()),
            NodeValue::Null => Some("null".to_string()),
            _ => None,
        }
    }

    pub fn as_number(&self) -> Option<f64> {
        match self {
            NodeValue::Number(n) => Some(*n),
            NodeValue::String(s) => s.parse().ok(),
            NodeValue::Boolean(b) => Some(if *b { 1.0 } else { 0.0 }),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            NodeValue::Boolean(b) => Some(*b),
            NodeValue::String(s) => Some(!s.is_empty()),
            NodeValue::Number(n) => Some(*n != 0.0),
            NodeValue::Null => Some(false),
            _ => Some(true),
        }
    }

    pub fn preview(&self, max_len: usize) -> String {
        let s = match self {
            NodeValue::String(s) => s.clone(),
            NodeValue::Number(n) => n.to_string(),
            NodeValue::Boolean(b) => b.to_string(),
            NodeValue::Null => "null".to_string(),
            NodeValue::Array(arr) => format!("Array[{}]", arr.len()),
            NodeValue::Object(obj) => format!("Object{{{} keys}}", obj.len()),
            NodeValue::File { path } => format!("File: {path}"),
        };
        if s.len() > max_len {
            format!("{}...", &s[..max_len])
        } else {
            s
        }
    }

    pub fn to_json_value(&self) -> serde_json::Value {
        match self {
            NodeValue::Null => serde_json::Value::Null,
            NodeValue::Boolean(b) => serde_json::Value::Bool(*b),
            NodeValue::Number(n) => serde_json::json!(n),
            NodeValue::String(s) => serde_json::Value::String(s.clone()),
            NodeValue::Array(arr) => {
                serde_json::Value::Array(arr.iter().map(|v| v.to_json_value()).collect())
            }
            NodeValue::Object(obj) => {
                serde_json::Value::Object(obj.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
            }
            NodeValue::File { path } => serde_json::json!({ "path": path }),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ExecutionEvent {
    NodeStarted {
        node_id: String,
    },
    NodeCompleted {
        node_id: String,
        output_preview: String,
        output_data: Option<serde_json::Value>,
        duration_ms: u64,
    },
    NodeError {
        node_id: String,
        error: String,
    },
    ExecutionComplete {
        total_duration_ms: u64,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub success: bool,
    pub total_duration_ms: u64,
    pub node_results: HashMap<String, NodeResult>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeResult {
    pub success: bool,
    pub output_preview: Option<String>,
    pub error: Option<String>,
    pub duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeDefinition {
    #[serde(rename = "type")]
    pub node_type: String,
    pub label: String,
    pub category: String,
    pub description: String,
}
