/// Adapters for mapping LangGraph graph structures into Ancora pipelines.
///
/// LangGraph models computation as a directed graph of nodes. This module
/// provides a lightweight representation and a mapper that converts it into
/// an ordered Ancora stage list.

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct LangGraphNode {
    pub id: String,
    pub label: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LangGraphEdge {
    pub from: String,
    pub to: String,
}

#[derive(Debug, Clone)]
pub struct LangGraphDefinition {
    pub nodes: Vec<LangGraphNode>,
    pub edges: Vec<LangGraphEdge>,
    pub entry: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AncoraStage {
    pub order: usize,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GraphMappingError {
    EntryNotFound(String),
    CycleDetected,
    DisconnectedNode(String),
}

impl std::fmt::Display for GraphMappingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EntryNotFound(e) => write!(f, "entry node not found: {}", e),
            Self::CycleDetected => write!(f, "cycle detected in graph"),
            Self::DisconnectedNode(n) => write!(f, "disconnected node: {}", n),
        }
    }
}

/// Topologically sort the LangGraph DAG starting from the entry node.
/// Returns an ordered list of Ancora stages.
pub fn map_langgraph_to_stages(
    graph: &LangGraphDefinition,
) -> Result<Vec<AncoraStage>, GraphMappingError> {
    // Validate entry node exists.
    if !graph.nodes.iter().any(|n| n.id == graph.entry) {
        return Err(GraphMappingError::EntryNotFound(graph.entry.clone()));
    }

    // Build adjacency list.
    let mut adj: HashMap<&str, Vec<&str>> = HashMap::new();
    for node in &graph.nodes {
        adj.entry(&node.id).or_default();
    }
    for edge in &graph.edges {
        adj.entry(&edge.from).or_default().push(&edge.to);
    }

    // Kahn's algorithm for topological sort.
    let mut in_degree: HashMap<&str, usize> = HashMap::new();
    for node in &graph.nodes {
        in_degree.entry(&node.id).or_insert(0);
    }
    for edge in &graph.edges {
        *in_degree.entry(&edge.to).or_insert(0) += 1;
    }

    let mut queue: Vec<&str> = in_degree
        .iter()
        .filter(|(_, &d)| d == 0)
        .map(|(&id, _)| id)
        .collect();
    queue.sort(); // deterministic order

    let mut sorted: Vec<&str> = Vec::new();
    while !queue.is_empty() {
        queue.sort();
        let node = queue.remove(0);
        sorted.push(node);
        if let Some(neighbors) = adj.get(node) {
            for &nb in neighbors {
                let deg = in_degree.get_mut(nb).unwrap();
                *deg -= 1;
                if *deg == 0 {
                    queue.push(nb);
                }
            }
        }
    }

    if sorted.len() != graph.nodes.len() {
        return Err(GraphMappingError::CycleDetected);
    }

    // Build label lookup.
    let label_map: HashMap<&str, &str> = graph
        .nodes
        .iter()
        .map(|n| (n.id.as_str(), n.label.as_str()))
        .collect();

    let stages = sorted
        .into_iter()
        .enumerate()
        .map(|(i, id)| AncoraStage {
            order: i,
            name: label_map.get(id).unwrap_or(&id).to_string(),
        })
        .collect();

    Ok(stages)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_graph_maps_correctly() {
        let g = LangGraphDefinition {
            nodes: vec![
                LangGraphNode { id: "a".into(), label: "Start".into() },
                LangGraphNode { id: "b".into(), label: "Middle".into() },
                LangGraphNode { id: "c".into(), label: "End".into() },
            ],
            edges: vec![
                LangGraphEdge { from: "a".into(), to: "b".into() },
                LangGraphEdge { from: "b".into(), to: "c".into() },
            ],
            entry: "a".into(),
        };
        let stages = map_langgraph_to_stages(&g).unwrap();
        assert_eq!(stages.len(), 3);
        assert_eq!(stages[0].name, "Start");
        assert_eq!(stages[2].name, "End");
    }
}
