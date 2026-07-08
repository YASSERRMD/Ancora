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
    /// Pauses the run until a human provides a decision.
    AwaitHuman,
}

impl NodeKind {
    pub fn to_str(self) -> &'static str {
        match self {
            NodeKind::Agent => "agent",
            NodeKind::Function => "function",
            NodeKind::Subgraph => "subgraph",
            NodeKind::AwaitHuman => "await-human",
        }
    }
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
    /// Explicit model override for this node; overrides the spec's model_id when set.
    pub model_id: Option<String>,
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

#[cfg(test)]
mod tests {
    use super::*;

    fn agent_node(id: &str) -> Node {
        Node {
            id: id.to_string(),
            kind: NodeKind::Agent,
            model_id: None,
            spec: NodeSpec::Agent(ancora_proto::ancora::AgentSpec {
                name: id.to_string(),
                model_id: "mock".to_string(),
                instructions: String::new(),
                output_schema_json: String::new(),
                tools: vec![],
                max_steps: 1,
                model_retry: None,
                model_params_json: String::new(),
            }),
        }
    }

    fn edge(from: &str, to: &str) -> Edge {
        Edge {
            from: from.to_string(),
            to: to.to_string(),
            condition: None,
        }
    }

    #[test]
    fn malformed_graphs_are_rejected() {
        // Empty nodes
        let empty = Graph {
            id: "g1".to_string(),
            nodes: vec![],
            edges: vec![],
            entry_node: "a".to_string(),
        };
        assert!(matches!(
            empty.validate(),
            Err(AncoraError::GraphInvalid(_))
        ));

        // Entry node missing
        let missing_entry = Graph {
            id: "g2".to_string(),
            nodes: vec![agent_node("a")],
            edges: vec![],
            entry_node: "missing".to_string(),
        };
        assert!(matches!(
            missing_entry.validate(),
            Err(AncoraError::GraphInvalid(_))
        ));

        // Duplicate node ids
        let dupe = Graph {
            id: "g3".to_string(),
            nodes: vec![agent_node("a"), agent_node("a")],
            edges: vec![],
            entry_node: "a".to_string(),
        };
        assert!(matches!(dupe.validate(), Err(AncoraError::GraphInvalid(_))));

        // Edge source does not exist
        let bad_src = Graph {
            id: "g4".to_string(),
            nodes: vec![agent_node("a")],
            edges: vec![edge("ghost", "a")],
            entry_node: "a".to_string(),
        };
        assert!(matches!(
            bad_src.validate(),
            Err(AncoraError::GraphInvalid(_))
        ));

        // Edge target does not exist
        let bad_tgt = Graph {
            id: "g5".to_string(),
            nodes: vec![agent_node("a")],
            edges: vec![edge("a", "ghost")],
            entry_node: "a".to_string(),
        };
        assert!(matches!(
            bad_tgt.validate(),
            Err(AncoraError::GraphInvalid(_))
        ));

        // Valid graph passes
        let valid = Graph {
            id: "g6".to_string(),
            nodes: vec![agent_node("a"), agent_node("b")],
            edges: vec![edge("a", "b")],
            entry_node: "a".to_string(),
        };
        valid.validate().unwrap();
    }
}
