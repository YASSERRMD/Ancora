/// import - Import an existing graph spec into the canvas model.

use crate::edges::{EdgeStore, EdgeType};
use crate::placement::{Canvas, CanvasNode};
use crate::scaffold::{Id, Position};
use std::collections::HashMap;

/// A serialized node in a graph spec file.
#[derive(Debug, Clone)]
pub struct SpecNode {
    pub id: String,
    pub kind: String,
    pub label: String,
    pub x: f64,
    pub y: f64,
    pub config: HashMap<String, String>,
}

/// A serialized edge in a graph spec file.
#[derive(Debug, Clone)]
pub struct SpecEdge {
    pub id: String,
    pub source: String,
    pub target: String,
    pub edge_type: String,
    pub label: Option<String>,
}

/// A complete serialized graph spec.
#[derive(Debug, Clone, Default)]
pub struct GraphSpec {
    pub name: String,
    pub version: u32,
    pub nodes: Vec<SpecNode>,
    pub edges: Vec<SpecEdge>,
    pub meta: HashMap<String, String>,
}

impl GraphSpec {
    pub fn new(name: impl Into<String>) -> Self {
        GraphSpec {
            name: name.into(),
            version: 1,
            nodes: Vec::new(),
            edges: Vec::new(),
            meta: HashMap::new(),
        }
    }
}

/// Result of a successful import: populated canvas and edge store.
pub struct ImportResult {
    pub canvas: Canvas,
    pub edges: EdgeStore,
    pub warnings: Vec<String>,
}

/// Errors that can occur during import.
#[derive(Debug, Clone, PartialEq)]
pub enum ImportError {
    DuplicateNodeId(String),
    UnknownEdgeSource(String),
    UnknownEdgeTarget(String),
    MalformedSpec(String),
}

impl std::fmt::Display for ImportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImportError::DuplicateNodeId(id) => write!(f, "duplicate node id: {}", id),
            ImportError::UnknownEdgeSource(id) => write!(f, "edge references unknown source node: {}", id),
            ImportError::UnknownEdgeTarget(id) => write!(f, "edge references unknown target node: {}", id),
            ImportError::MalformedSpec(msg) => write!(f, "malformed spec: {}", msg),
        }
    }
}

/// Import a GraphSpec into canvas and edge store representations.
pub fn import_spec(spec: GraphSpec) -> Result<ImportResult, ImportError> {
    if spec.name.trim().is_empty() {
        return Err(ImportError::MalformedSpec("spec name must not be empty".into()));
    }

    let mut canvas = Canvas::new();
    let mut edges = EdgeStore::new();
    let mut warnings = Vec::new();
    let mut known_ids: HashMap<String, String> = HashMap::new(); // id -> kind

    // Import nodes
    for sn in &spec.nodes {
        if known_ids.contains_key(&sn.id) {
            return Err(ImportError::DuplicateNodeId(sn.id.clone()));
        }
        let node = CanvasNode {
            id: Id::new(sn.id.clone()),
            kind: sn.kind.clone(),
            label: sn.label.clone(),
            position: Position::new(sn.x, sn.y),
            size: Default::default(),
            config: sn.config.clone(),
        };
        canvas
            .place_node_with_id(node)
            .map_err(|e| ImportError::MalformedSpec(e.to_string()))?;
        known_ids.insert(sn.id.clone(), sn.kind.clone());
    }

    // Import edges
    for se in &spec.edges {
        let source_kind = known_ids
            .get(&se.source)
            .ok_or_else(|| ImportError::UnknownEdgeSource(se.source.clone()))?
            .clone();
        let target_kind = known_ids
            .get(&se.target)
            .ok_or_else(|| ImportError::UnknownEdgeTarget(se.target.clone()))?
            .clone();

        let et = EdgeType::from_str(&se.edge_type);
        match edges.add_edge(
            Id::new(se.source.clone()),
            &source_kind,
            Id::new(se.target.clone()),
            &target_kind,
            et,
        ) {
            Ok(_) => {}
            Err(e) => warnings.push(format!("edge {}: {}", se.id, e)),
        }
    }

    Ok(ImportResult { canvas, edges, warnings })
}

/// Parse a simple key=value line-based spec format for testing.
/// Format:
///   node <id> <kind> <label> <x> <y>
///   edge <id> <source> <target> <type>
///   name <name>
pub fn parse_simple_spec(text: &str) -> Result<GraphSpec, ImportError> {
    let mut spec = GraphSpec::default();
    for (lineno, line) in text.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let parts: Vec<&str> = line.splitn(7, ' ').collect();
        match parts.as_slice() {
            ["name", rest @ ..] => {
                spec.name = rest.join(" ");
            }
            ["node", id, kind, label, x, y] => {
                let x = x.parse::<f64>().map_err(|_| {
                    ImportError::MalformedSpec(format!("line {}: invalid x coordinate", lineno + 1))
                })?;
                let y = y.parse::<f64>().map_err(|_| {
                    ImportError::MalformedSpec(format!("line {}: invalid y coordinate", lineno + 1))
                })?;
                spec.nodes.push(SpecNode {
                    id: id.to_string(),
                    kind: kind.to_string(),
                    label: label.to_string(),
                    x,
                    y,
                    config: HashMap::new(),
                });
            }
            ["edge", id, source, target, edge_type] => {
                spec.edges.push(SpecEdge {
                    id: id.to_string(),
                    source: source.to_string(),
                    target: target.to_string(),
                    edge_type: edge_type.to_string(),
                    label: None,
                });
            }
            _ => {
                return Err(ImportError::MalformedSpec(format!(
                    "line {}: unrecognized directive: {}",
                    lineno + 1,
                    line
                )));
            }
        }
    }
    Ok(spec)
}

#[cfg(test)]
mod unit {
    use super::*;

    fn make_spec() -> GraphSpec {
        let mut spec = GraphSpec::new("test_graph");
        spec.nodes.push(SpecNode {
            id: "n1".into(),
            kind: "agent.llm".into(),
            label: "LLM".into(),
            x: 0.0,
            y: 0.0,
            config: HashMap::new(),
        });
        spec.nodes.push(SpecNode {
            id: "n2".into(),
            kind: "verifier.json_schema".into(),
            label: "Verify".into(),
            x: 200.0,
            y: 0.0,
            config: HashMap::new(),
        });
        spec.edges.push(SpecEdge {
            id: "e1".into(),
            source: "n1".into(),
            target: "n2".into(),
            edge_type: "data_flow".into(),
            label: None,
        });
        spec
    }

    #[test]
    fn import_renders_nodes() {
        let spec = make_spec();
        let result = import_spec(spec).unwrap();
        assert_eq!(result.canvas.node_count(), 2);
    }

    #[test]
    fn import_renders_edges() {
        let spec = make_spec();
        let result = import_spec(spec).unwrap();
        assert_eq!(result.edges.edge_count(), 1);
    }

    #[test]
    fn import_unknown_edge_source_rejected() {
        let mut spec = make_spec();
        spec.edges.push(SpecEdge {
            id: "e2".into(),
            source: "nonexistent".into(),
            target: "n2".into(),
            edge_type: "data_flow".into(),
            label: None,
        });
        let result = import_spec(spec);
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert!(matches!(err, ImportError::UnknownEdgeSource(_)));
    }

    #[test]
    fn parse_simple_spec_works() {
        let text = "name my_graph\nnode n1 agent.llm Agent 0 0\nnode n2 tool.web_search Search 100 0\nedge e1 n1 n2 data_flow";
        let spec = parse_simple_spec(text).unwrap();
        assert_eq!(spec.name, "my_graph");
        assert_eq!(spec.nodes.len(), 2);
        assert_eq!(spec.edges.len(), 1);
    }
}
