use crate::error::AncoraError;

/// A directed connection between two nodes.
pub struct Edge {
    /// Source node id.
    pub from: String,
    /// Target node id.
    pub to: String,
    /// Optional condition expression evaluated at runtime to decide if this edge is taken.
    pub condition: Option<String>,
}

/// A single vertex in the orchestration graph.
pub struct Node {
    pub id: String,
}

/// The complete orchestration graph.
pub struct Graph {
    pub id: String,
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    /// Id of the node to execute first.
    pub entry_node: String,
}
