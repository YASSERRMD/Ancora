use ancora_proto::ancora::AgentSpec;

use crate::error::AncoraError;

/// The kind of work a node performs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeKind {
    /// Runs a single-agent reason-act loop.
    Agent,
    /// Calls a named function registered with the executor.
    Function,
    /// Delegates to a nested `Graph`.
    Subgraph,
}

/// Configuration specific to each node kind.
pub enum NodeSpec {
    Agent(AgentSpec),
    Function { name: String },
    Subgraph { graph_id: String },
}

/// A single vertex in the orchestration graph.
pub struct Node {
    pub id: String,
    pub kind: NodeKind,
    pub spec: NodeSpec,
}

/// A directed connection between two nodes.
pub struct Edge {
    /// Source node id.
    pub from: String,
    /// Target node id.
    pub to: String,
    /// Optional condition expression evaluated at runtime to decide if this edge is taken.
    pub condition: Option<String>,
}

/// The complete orchestration graph.
pub struct Graph {
    pub id: String,
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    /// Id of the node to execute first.
    pub entry_node: String,
}
