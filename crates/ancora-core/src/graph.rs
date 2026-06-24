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

impl Graph {
    /// Check that the graph satisfies structural invariants:
    /// - At least one node
    /// - No duplicate node ids
    /// - Entry node exists
    /// - Every edge source and target refer to an existing node
    pub fn validate(&self) -> Result<(), AncoraError> {
        if self.nodes.is_empty() {
            return Err(AncoraError::GraphInvalid("graph has no nodes".to_string()));
        }

        // Check for duplicate ids
        let mut seen = std::collections::HashSet::new();
        for node in &self.nodes {
            if !seen.insert(node.id.as_str()) {
                return Err(AncoraError::GraphInvalid(format!(
                    "duplicate node id '{}'",
                    node.id
                )));
            }
        }

        // Entry node must exist
        if !seen.contains(self.entry_node.as_str()) {
            return Err(AncoraError::GraphInvalid(format!(
                "entry node '{}' does not exist",
                self.entry_node
            )));
        }

        // Every edge must reference existing nodes
        for edge in &self.edges {
            if !seen.contains(edge.from.as_str()) {
                return Err(AncoraError::GraphInvalid(format!(
                    "edge source '{}' does not exist",
                    edge.from
                )));
            }
            if !seen.contains(edge.to.as_str()) {
                return Err(AncoraError::GraphInvalid(format!(
                    "edge target '{}' does not exist",
                    edge.to
                )));
            }
        }

        Ok(())
    }
}
