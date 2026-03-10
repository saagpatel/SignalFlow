use petgraph::algo::toposort;
use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::HashMap;

use crate::error::AppError;
use crate::types::{FlowDocument, FlowEdge};

pub struct FlowGraph {
    pub graph: DiGraph<String, FlowEdge>,
    pub node_indices: HashMap<String, NodeIndex>,
    pub execution_order: Vec<String>,
    pub execution_layers: Vec<Vec<String>>,
}

impl FlowGraph {
    pub fn from_document(doc: &FlowDocument) -> Result<Self, AppError> {
        let mut graph = DiGraph::new();
        let mut node_indices = HashMap::new();

        // Add nodes
        for node in &doc.nodes {
            let idx = graph.add_node(node.id.clone());
            node_indices.insert(node.id.clone(), idx);
        }

        // Add edges
        for edge in &doc.edges {
            let source_idx = node_indices
                .get(&edge.source)
                .ok_or_else(|| AppError::Graph(format!("Source node {} not found", edge.source)))?;
            let target_idx = node_indices
                .get(&edge.target)
                .ok_or_else(|| AppError::Graph(format!("Target node {} not found", edge.target)))?;
            graph.add_edge(*source_idx, *target_idx, edge.clone());
        }

        // Topological sort
        let sorted = toposort(&graph, None).map_err(|_| AppError::CycleDetected)?;

        let execution_order: Vec<String> = sorted.iter().map(|idx| graph[*idx].clone()).collect();

        // Build execution layers (nodes at same depth can run in parallel)
        let layers = build_layers(&graph, &sorted, &node_indices);

        Ok(FlowGraph {
            graph,
            node_indices,
            execution_order,
            execution_layers: layers,
        })
    }

    pub fn get_input_edges(&self, node_id: &str) -> Vec<&FlowEdge> {
        let Some(idx) = self.node_indices.get(node_id) else {
            return vec![];
        };
        self.graph
            .edges_directed(*idx, petgraph::Direction::Incoming)
            .map(|e| e.weight())
            .collect()
    }
}

fn build_layers(
    graph: &DiGraph<String, FlowEdge>,
    sorted: &[NodeIndex],
    _node_indices: &HashMap<String, NodeIndex>,
) -> Vec<Vec<String>> {
    let mut depth: HashMap<NodeIndex, usize> = HashMap::new();
    let mut max_depth = 0;

    for &idx in sorted {
        let d = graph
            .neighbors_directed(idx, petgraph::Direction::Incoming)
            .map(|parent| depth.get(&parent).copied().unwrap_or(0) + 1)
            .max()
            .unwrap_or(0);
        depth.insert(idx, d);
        max_depth = max_depth.max(d);
    }

    let mut layers: Vec<Vec<String>> = vec![vec![]; max_depth + 1];
    for &idx in sorted {
        let d = depth[&idx];
        layers[d].push(graph[idx].clone());
    }

    layers
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::*;

    fn make_doc(nodes: Vec<(&str, &str)>, edges: Vec<(&str, &str)>) -> FlowDocument {
        FlowDocument {
            id: None,
            name: "test".to_string(),
            nodes: nodes
                .into_iter()
                .map(|(id, t)| FlowNode {
                    id: id.to_string(),
                    node_type: t.to_string(),
                    position: Position::default(),
                    data: serde_json::Value::Object(serde_json::Map::new()),
                })
                .collect(),
            edges: edges
                .into_iter()
                .enumerate()
                .map(|(i, (s, t))| FlowEdge {
                    id: format!("e{i}"),
                    source: s.to_string(),
                    target: t.to_string(),
                    source_handle: Some("value".to_string()),
                    target_handle: Some("input".to_string()),
                })
                .collect(),
            viewport: Viewport::default(),
        }
    }

    #[test]
    fn test_toposort_linear() {
        let doc = make_doc(
            vec![
                ("a", "textInput"),
                ("b", "textTemplate"),
                ("c", "debug"),
                ("d", "filter"),
                ("e", "debug"),
            ],
            vec![("a", "b"), ("b", "c"), ("c", "d"), ("d", "e")],
        );
        let graph = FlowGraph::from_document(&doc).unwrap();
        assert_eq!(graph.execution_order, vec!["a", "b", "c", "d", "e"]);
    }

    #[test]
    fn test_cycle_detection() {
        let doc = make_doc(
            vec![("a", "textInput"), ("b", "debug")],
            vec![("a", "b"), ("b", "a")],
        );
        let result = FlowGraph::from_document(&doc);
        assert!(result.is_err());
    }

    #[test]
    fn test_parallel_layers() {
        // a → c, b → c (a and b can run in parallel)
        let doc = make_doc(
            vec![("a", "textInput"), ("b", "textInput"), ("c", "merge")],
            vec![("a", "c"), ("b", "c")],
        );
        let graph = FlowGraph::from_document(&doc).unwrap();
        assert_eq!(graph.execution_layers.len(), 2);
        assert_eq!(graph.execution_layers[0].len(), 2); // a and b
        assert_eq!(graph.execution_layers[1].len(), 1); // c
    }
}
