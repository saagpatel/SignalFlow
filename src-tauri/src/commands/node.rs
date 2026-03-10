use std::collections::HashSet;

use crate::error::AppError;
use crate::nodes::registry::NodeRegistry;
use crate::types::NodeDefinition;
use serde::Deserialize;

#[derive(Deserialize)]
struct NodeCatalogFile {
    definitions: Vec<NodeDefinition>,
}

#[tauri::command]
pub async fn get_node_definitions() -> Result<Vec<NodeDefinition>, AppError> {
    let registry = NodeRegistry::default();
    let supported_types: HashSet<&str> = registry.supported_node_types().into_iter().collect();

    Ok(load_node_catalog()?
        .definitions
        .into_iter()
        .filter(|definition| supported_types.contains(definition.node_type.as_str()))
        .collect())
}

fn load_node_catalog() -> Result<NodeCatalogFile, AppError> {
    serde_json::from_str(include_str!("../../../src/shared/nodeCatalog.json"))
        .map_err(|error| AppError::Other(format!("Failed to parse shared node catalog: {error}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shared_catalog_covers_control_and_ai_nodes() {
        let registry = NodeRegistry::default();
        let supported_types: HashSet<&str> = registry.supported_node_types().into_iter().collect();
        let definitions: Vec<NodeDefinition> = load_node_catalog()
            .unwrap()
            .definitions
            .into_iter()
            .filter(|definition| supported_types.contains(definition.node_type.as_str()))
            .collect();
        let node_types: HashSet<&str> = definitions
            .iter()
            .map(|definition| definition.node_type.as_str())
            .collect();

        assert!(node_types.contains("forEach"));
        assert!(node_types.contains("tryCatch"));
        assert!(node_types.contains("llmPrompt"));
        assert!(node_types.contains("llmChat"));
        assert!(node_types.contains("code"));
    }
}
