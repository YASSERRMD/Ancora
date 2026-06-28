//! Graph builder export time measurement.
//!
//! Models the time required to serialise a completed agent graph into an
//! exportable representation. The graph is a directed acyclic structure of
//! nodes and edges represented purely with standard-library types.

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// A node in the agent graph.
#[derive(Debug, Clone)]
pub struct GraphNode {
    /// Unique identifier for this node.
    pub id: String,
    /// Human-readable label.
    pub label: String,
    /// Arbitrary key-value metadata.
    pub metadata: HashMap<String, String>,
}

impl GraphNode {
    /// Construct a new node with no metadata.
    pub fn new(id: &str, label: &str) -> Self {
        Self {
            id: id.to_owned(),
            label: label.to_owned(),
            metadata: HashMap::new(),
        }
    }
}

/// A directed edge between two graph nodes.
#[derive(Debug, Clone)]
pub struct GraphEdge {
    /// ID of the source node.
    pub from: String,
    /// ID of the target node.
    pub to: String,
    /// Optional edge weight.
    pub weight: f64,
}

impl GraphEdge {
    /// Construct a new edge with unit weight.
    pub fn new(from: &str, to: &str) -> Self {
        Self {
            from: from.to_owned(),
            to: to.to_owned(),
            weight: 1.0,
        }
    }
}

/// An in-memory agent graph.
#[derive(Debug, Default)]
pub struct AgentGraph {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
}

impl AgentGraph {
    /// Create an empty graph.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a node to the graph.
    pub fn add_node(&mut self, node: GraphNode) {
        self.nodes.push(node);
    }

    /// Add an edge to the graph.
    pub fn add_edge(&mut self, edge: GraphEdge) {
        self.edges.push(edge);
    }
}

/// A serialised representation of a graph for export.
#[derive(Debug, Clone)]
pub struct ExportedGraph {
    /// Serialised form (newline-delimited records for simplicity).
    pub content: String,
    /// Node count at export time.
    pub node_count: usize,
    /// Edge count at export time.
    pub edge_count: usize,
}

/// Result of a graph export operation.
#[derive(Debug)]
pub struct ExportResult {
    /// The exported graph data.
    pub exported: ExportedGraph,
    /// Total elapsed time.
    pub elapsed: Duration,
}

/// Export an agent graph to a text-based representation.
pub fn export_graph(graph: &AgentGraph) -> ExportResult {
    let start = Instant::now();

    let mut lines: Vec<String> = Vec::with_capacity(graph.nodes.len() + graph.edges.len() + 2);

    lines.push(format!("nodes:{}", graph.nodes.len()));
    for n in &graph.nodes {
        lines.push(format!("N {} {}", n.id, n.label));
    }

    lines.push(format!("edges:{}", graph.edges.len()));
    for e in &graph.edges {
        lines.push(format!("E {} {} {:.4}", e.from, e.to, e.weight));
    }

    let content = lines.join("\n");

    ExportResult {
        exported: ExportedGraph {
            content,
            node_count: graph.nodes.len(),
            edge_count: graph.edges.len(),
        },
        elapsed: start.elapsed(),
    }
}

/// Regression threshold for exporting a graph in microseconds.
pub const EXPORT_TARGET_US: u64 = 5_000;

/// Returns `true` if the export completed within the regression threshold.
pub fn within_target(result: &ExportResult) -> bool {
    result.elapsed.as_micros() as u64 <= EXPORT_TARGET_US
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn export_counts_match() {
        let mut g = AgentGraph::new();
        g.add_node(GraphNode::new("n1", "Node 1"));
        g.add_node(GraphNode::new("n2", "Node 2"));
        g.add_edge(GraphEdge::new("n1", "n2"));
        let r = export_graph(&g);
        assert_eq!(r.exported.node_count, 2);
        assert_eq!(r.exported.edge_count, 1);
    }

    #[test]
    fn export_contains_node_ids() {
        let mut g = AgentGraph::new();
        g.add_node(GraphNode::new("alpha", "Alpha Node"));
        let r = export_graph(&g);
        assert!(r.exported.content.contains("alpha"));
    }
}
