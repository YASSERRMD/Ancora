/// edges - Typed edge connections between canvas nodes.
use crate::scaffold::Id;
use std::collections::HashMap;

/// The semantic type of a connection.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EdgeType {
    /// Standard data flow: output of one node feeds input of next.
    DataFlow,
    /// Control dependency: the target node only runs after the source.
    ControlDep,
    /// Verification: the source verifies the output of the target.
    Verification,
    /// Loop back-edge connecting a node to an earlier ancestor.
    LoopBack,
    /// Custom edge type defined by a plugin or template.
    Custom(String),
}

impl std::fmt::Display for EdgeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EdgeType::DataFlow => write!(f, "data_flow"),
            EdgeType::ControlDep => write!(f, "control_dep"),
            EdgeType::Verification => write!(f, "verification"),
            EdgeType::LoopBack => write!(f, "loop_back"),
            EdgeType::Custom(s) => write!(f, "custom:{}", s),
        }
    }
}

impl EdgeType {
    pub fn from_raw(s: &str) -> Self {
        match s {
            "data_flow" => EdgeType::DataFlow,
            "control_dep" => EdgeType::ControlDep,
            "verification" => EdgeType::Verification,
            "loop_back" => EdgeType::LoopBack,
            other => {
                if let Some(rest) = other.strip_prefix("custom:") {
                    EdgeType::Custom(rest.to_string())
                } else {
                    EdgeType::Custom(other.to_string())
                }
            }
        }
    }
}

/// A directed edge between two nodes.
#[derive(Debug, Clone)]
pub struct Edge {
    pub id: Id,
    pub source: Id,
    pub target: Id,
    pub edge_type: EdgeType,
    /// Optional label displayed in the builder UI.
    pub label: Option<String>,
    pub metadata: HashMap<String, String>,
}

impl Edge {
    pub fn new(id: Id, source: Id, target: Id, edge_type: EdgeType) -> Self {
        Edge {
            id,
            source,
            target,
            edge_type,
            label: None,
            metadata: HashMap::new(),
        }
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
}

/// Manages the set of edges on the canvas and enforces connection rules.
#[derive(Debug, Default, Clone)]
pub struct EdgeStore {
    edges: HashMap<Id, Edge>,
    next_id: u64,
    /// Per-kind rules: which target kinds are allowed for a given source kind.
    allowed_connections: HashMap<String, Vec<String>>,
}

impl EdgeStore {
    pub fn new() -> Self {
        EdgeStore::default()
    }

    fn gen_id(&mut self) -> Id {
        let id = Id::new(format!("edge_{}", self.next_id));
        self.next_id += 1;
        id
    }

    /// Register an allowed connection: source_kind -> target_kind.
    pub fn allow_connection(
        &mut self,
        source_kind: impl Into<String>,
        target_kind: impl Into<String>,
    ) {
        self.allowed_connections
            .entry(source_kind.into())
            .or_default()
            .push(target_kind.into());
    }

    /// Check whether a connection from source_kind to target_kind is allowed.
    /// If no rules are registered for the source kind, the connection is allowed by default.
    pub fn is_connection_allowed(&self, source_kind: &str, target_kind: &str) -> bool {
        match self.allowed_connections.get(source_kind) {
            None => true, // no restrictions
            Some(allowed) => allowed.iter().any(|k| k == target_kind || k == "*"),
        }
    }

    /// Add an edge. Returns an error if the connection violates registered rules or
    /// if a self-loop is attempted.
    pub fn add_edge(
        &mut self,
        source: Id,
        source_kind: &str,
        target: Id,
        target_kind: &str,
        edge_type: EdgeType,
    ) -> Result<Id, EdgeError> {
        if source == target {
            return Err(EdgeError::SelfLoop(source.0));
        }
        if !self.is_connection_allowed(source_kind, target_kind) {
            return Err(EdgeError::InvalidConnection {
                source_kind: source_kind.to_string(),
                target_kind: target_kind.to_string(),
            });
        }
        // Prevent duplicate edges (same source/target/type).
        for e in self.edges.values() {
            if e.source == source && e.target == target && e.edge_type == edge_type {
                return Err(EdgeError::DuplicateEdge);
            }
        }
        let id = self.gen_id();
        let edge = Edge::new(id.clone(), source, target, edge_type);
        self.edges.insert(id.clone(), edge);
        Ok(id)
    }

    pub fn remove_edge(&mut self, id: &Id) -> Option<Edge> {
        self.edges.remove(id)
    }

    pub fn get_edge(&self, id: &Id) -> Option<&Edge> {
        self.edges.get(id)
    }

    pub fn all_edges(&self) -> impl Iterator<Item = &Edge> {
        self.edges.values()
    }

    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Return all edges that originate from a given node.
    pub fn edges_from(&self, source: &Id) -> Vec<&Edge> {
        self.edges
            .values()
            .filter(|e| &e.source == source)
            .collect()
    }

    /// Return all edges that terminate at a given node.
    pub fn edges_to(&self, target: &Id) -> Vec<&Edge> {
        self.edges
            .values()
            .filter(|e| &e.target == target)
            .collect()
    }

    /// Remove all edges connected to a node (call when removing a node).
    pub fn remove_edges_for_node(&mut self, node_id: &Id) {
        self.edges
            .retain(|_, e| &e.source != node_id && &e.target != node_id);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EdgeError {
    SelfLoop(String),
    InvalidConnection {
        source_kind: String,
        target_kind: String,
    },
    DuplicateEdge,
}

impl std::fmt::Display for EdgeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EdgeError::SelfLoop(id) => write!(f, "self-loop on node {}", id),
            EdgeError::InvalidConnection {
                source_kind,
                target_kind,
            } => {
                write!(
                    f,
                    "connection from '{}' to '{}' is not allowed",
                    source_kind, target_kind
                )
            }
            EdgeError::DuplicateEdge => write!(f, "duplicate edge"),
        }
    }
}

#[cfg(test)]
mod unit {
    use super::*;

    #[test]
    fn add_edge_succeeds() {
        let mut store = EdgeStore::new();
        let id = store
            .add_edge(
                Id::new("a"),
                "agent.llm",
                Id::new("b"),
                "verifier.json_schema",
                EdgeType::DataFlow,
            )
            .unwrap();
        assert!(store.get_edge(&id).is_some());
    }

    #[test]
    fn self_loop_rejected() {
        let mut store = EdgeStore::new();
        let err = store
            .add_edge(
                Id::new("a"),
                "agent.llm",
                Id::new("a"),
                "agent.llm",
                EdgeType::DataFlow,
            )
            .unwrap_err();
        assert!(matches!(err, EdgeError::SelfLoop(_)));
    }

    #[test]
    fn invalid_connection_rejected() {
        let mut store = EdgeStore::new();
        // Only allow agent -> verifier
        store.allow_connection("agent.llm", "verifier.json_schema");
        let err = store
            .add_edge(
                Id::new("a"),
                "agent.llm",
                Id::new("b"),
                "agent.llm", // not allowed
                EdgeType::DataFlow,
            )
            .unwrap_err();
        assert!(matches!(err, EdgeError::InvalidConnection { .. }));
    }

    #[test]
    fn duplicate_edge_rejected() {
        let mut store = EdgeStore::new();
        store
            .add_edge(
                Id::new("a"),
                "agent.llm",
                Id::new("b"),
                "tool.web_search",
                EdgeType::DataFlow,
            )
            .unwrap();
        let err = store
            .add_edge(
                Id::new("a"),
                "agent.llm",
                Id::new("b"),
                "tool.web_search",
                EdgeType::DataFlow,
            )
            .unwrap_err();
        assert_eq!(err, EdgeError::DuplicateEdge);
    }

    #[test]
    fn edge_type_round_trip() {
        let t = EdgeType::Custom("my_type".into());
        let s = t.to_string();
        let t2 = EdgeType::from_raw(&s);
        assert_eq!(t, t2);
    }
}
