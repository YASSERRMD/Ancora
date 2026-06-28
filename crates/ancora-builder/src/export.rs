/// export - Export the canvas model to a graph spec.

use crate::edges::EdgeStore;
use crate::import::{GraphSpec, SpecEdge, SpecNode};
use crate::placement::Canvas;
use std::collections::HashMap;

/// Export options controlling what to include.
#[derive(Debug, Clone)]
pub struct ExportOptions {
    /// Include node position information in the spec.
    pub include_positions: bool,
    /// Include edge labels in the spec.
    pub include_edge_labels: bool,
    /// Extra metadata to embed in the spec.
    pub extra_meta: HashMap<String, String>,
}

impl Default for ExportOptions {
    fn default() -> Self {
        ExportOptions {
            include_positions: true,
            include_edge_labels: true,
            extra_meta: HashMap::new(),
        }
    }
}

/// Export errors.
#[derive(Debug, Clone, PartialEq)]
pub enum ExportError {
    EmptyGraph,
    SerializationFailed(String),
}

impl std::fmt::Display for ExportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExportError::EmptyGraph => write!(f, "cannot export an empty graph"),
            ExportError::SerializationFailed(msg) => write!(f, "serialization failed: {}", msg),
        }
    }
}

/// Serialize the canvas and edge store to a GraphSpec.
pub fn export_spec(
    name: impl Into<String>,
    canvas: &Canvas,
    edges: &EdgeStore,
    opts: &ExportOptions,
) -> Result<GraphSpec, ExportError> {
    if canvas.node_count() == 0 {
        return Err(ExportError::EmptyGraph);
    }

    let mut spec = GraphSpec::new(name);
    spec.version = 1;
    spec.meta = opts.extra_meta.clone();

    // Serialize nodes (sorted by id for determinism).
    let mut nodes: Vec<SpecNode> = canvas
        .all_nodes()
        .map(|n| SpecNode {
            id: n.id.0.clone(),
            kind: n.kind.clone(),
            label: n.label.clone(),
            x: if opts.include_positions { n.position.x } else { 0.0 },
            y: if opts.include_positions { n.position.y } else { 0.0 },
            config: n.config.clone(),
        })
        .collect();
    nodes.sort_by(|a, b| a.id.cmp(&b.id));
    spec.nodes = nodes;

    // Serialize edges (sorted by id for determinism).
    let mut spec_edges: Vec<SpecEdge> = edges
        .all_edges()
        .map(|e| SpecEdge {
            id: e.id.0.clone(),
            source: e.source.0.clone(),
            target: e.target.0.clone(),
            edge_type: e.edge_type.to_string(),
            label: if opts.include_edge_labels { e.label.clone() } else { None },
        })
        .collect();
    spec_edges.sort_by(|a, b| a.id.cmp(&b.id));
    spec.edges = spec_edges;

    Ok(spec)
}

/// Serialize a GraphSpec to the simple line-based text format.
pub fn spec_to_text(spec: &GraphSpec) -> String {
    let mut lines = Vec::new();
    lines.push(format!("name {}", spec.name));
    for n in &spec.nodes {
        // Replace spaces in label with underscores so the line-based format stays parseable.
        let safe_label = n.label.replace(' ', "_");
        lines.push(format!("node {} {} {} {} {}", n.id, n.kind, safe_label, n.x, n.y));
    }
    for e in &spec.edges {
        lines.push(format!("edge {} {} {} {}", e.id, e.source, e.target, e.edge_type));
    }
    lines.join("\n")
}

#[cfg(test)]
mod unit {
    use super::*;
    use crate::edges::EdgeType;
    use crate::import::import_spec;
    use crate::placement::Canvas;
    use crate::scaffold::{Id, Position};

    fn build_simple_canvas() -> (Canvas, EdgeStore) {
        let mut canvas = Canvas::new();
        let mut edges = EdgeStore::new();
        let a = canvas.place_node("agent.llm", "Agent", Position::new(0.0, 0.0));
        let b = canvas.place_node("verifier.json_schema", "Verifier", Position::new(200.0, 0.0));
        edges
            .add_edge(a.clone(), "agent.llm", b.clone(), "verifier.json_schema", EdgeType::DataFlow)
            .unwrap();
        (canvas, edges)
    }

    #[test]
    fn export_produces_spec() {
        let (canvas, edges) = build_simple_canvas();
        let spec = export_spec("my_graph", &canvas, &edges, &ExportOptions::default()).unwrap();
        assert_eq!(spec.nodes.len(), 2);
        assert_eq!(spec.edges.len(), 1);
    }

    #[test]
    fn export_empty_graph_fails() {
        let canvas = Canvas::new();
        let edges = EdgeStore::new();
        let err = export_spec("empty", &canvas, &edges, &ExportOptions::default()).unwrap_err();
        assert_eq!(err, ExportError::EmptyGraph);
    }

    #[test]
    fn spec_roundtrip_via_text() {
        let (canvas, edges) = build_simple_canvas();
        let spec = export_spec("rt_graph", &canvas, &edges, &ExportOptions::default()).unwrap();
        let text = spec_to_text(&spec);
        let spec2 = crate::import::parse_simple_spec(&text).unwrap();
        // Round-trip: node count should match
        assert_eq!(spec2.nodes.len(), spec.nodes.len());
        assert_eq!(spec2.edges.len(), spec.edges.len());
    }

    #[test]
    fn exported_spec_can_be_imported() {
        let (canvas, edges) = build_simple_canvas();
        let spec = export_spec("import_test", &canvas, &edges, &ExportOptions::default()).unwrap();
        let result = import_spec(spec).unwrap();
        assert_eq!(result.canvas.node_count(), 2);
        assert_eq!(result.edges.edge_count(), 1);
    }
}
