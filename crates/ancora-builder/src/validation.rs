/// validation - Validate graph specs and live canvas state in the editor.
use crate::edges::EdgeStore;
use crate::import::GraphSpec;
use crate::placement::Canvas;

/// A single validation diagnostic.
#[derive(Debug, Clone, PartialEq)]
pub struct Diagnostic {
    pub severity: Severity,
    pub message: String,
    /// The id of the node or edge this diagnostic pertains to, if applicable.
    pub target: Option<String>,
}

impl Diagnostic {
    pub fn error(message: impl Into<String>) -> Self {
        Diagnostic {
            severity: Severity::Error,
            message: message.into(),
            target: None,
        }
    }

    pub fn warning(message: impl Into<String>) -> Self {
        Diagnostic {
            severity: Severity::Warning,
            message: message.into(),
            target: None,
        }
    }

    pub fn info(message: impl Into<String>) -> Self {
        Diagnostic {
            severity: Severity::Info,
            message: message.into(),
            target: None,
        }
    }

    pub fn with_target(mut self, target: impl Into<String>) -> Self {
        self.target = Some(target.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Info,
    Warning,
    Error,
}

/// The aggregate result of a validation pass.
#[derive(Debug, Default, Clone)]
pub struct ValidationReport {
    pub diagnostics: Vec<Diagnostic>,
}

impl ValidationReport {
    pub fn new() -> Self {
        ValidationReport::default()
    }

    pub fn push(&mut self, d: Diagnostic) {
        self.diagnostics.push(d);
    }

    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|d| d.severity == Severity::Error)
    }

    pub fn errors(&self) -> impl Iterator<Item = &Diagnostic> {
        self.diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
    }

    pub fn warnings(&self) -> impl Iterator<Item = &Diagnostic> {
        self.diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Warning)
    }

    pub fn is_valid(&self) -> bool {
        !self.has_errors()
    }
}

/// Validate a serialized GraphSpec (used before import or before saving).
pub fn validate_spec(spec: &GraphSpec) -> ValidationReport {
    let mut report = ValidationReport::new();

    if spec.name.trim().is_empty() {
        report.push(Diagnostic::error("spec name must not be empty"));
    }

    if spec.nodes.is_empty() {
        report.push(Diagnostic::warning("graph has no nodes"));
    }

    // Check for duplicate node IDs.
    let mut seen_node_ids = std::collections::HashSet::new();
    for n in &spec.nodes {
        if n.id.trim().is_empty() {
            report.push(Diagnostic::error("node has an empty id").with_target(""));
        } else if !seen_node_ids.insert(n.id.clone()) {
            report.push(
                Diagnostic::error(format!("duplicate node id '{}'", n.id))
                    .with_target(n.id.clone()),
            );
        }
        if n.kind.trim().is_empty() {
            report.push(
                Diagnostic::error(format!("node '{}' has an empty kind", n.id))
                    .with_target(n.id.clone()),
            );
        }
    }

    // Check edge references.
    let mut seen_edge_ids = std::collections::HashSet::new();
    for e in &spec.edges {
        if e.id.trim().is_empty() {
            report.push(Diagnostic::error("edge has an empty id"));
        } else if !seen_edge_ids.insert(e.id.clone()) {
            report.push(
                Diagnostic::error(format!("duplicate edge id '{}'", e.id))
                    .with_target(e.id.clone()),
            );
        }
        if !seen_node_ids.contains(&e.source) {
            report.push(
                Diagnostic::error(format!(
                    "edge '{}' references unknown source node '{}'",
                    e.id, e.source
                ))
                .with_target(e.id.clone()),
            );
        }
        if !seen_node_ids.contains(&e.target) {
            report.push(
                Diagnostic::error(format!(
                    "edge '{}' references unknown target node '{}'",
                    e.id, e.target
                ))
                .with_target(e.id.clone()),
            );
        }
        if e.source == e.target {
            report.push(
                Diagnostic::error(format!("edge '{}' is a self-loop", e.id))
                    .with_target(e.id.clone()),
            );
        }
    }

    // Structural warning: any isolated node?
    for n in &spec.nodes {
        let connected = spec
            .edges
            .iter()
            .any(|e| e.source == n.id || e.target == n.id);
        if !connected && spec.nodes.len() > 1 {
            report.push(
                Diagnostic::warning(format!("node '{}' is isolated (no edges)", n.id))
                    .with_target(n.id.clone()),
            );
        }
    }

    report
}

/// Validate the live canvas/edge state in the editor.
pub fn validate_canvas(canvas: &Canvas, edges: &EdgeStore) -> ValidationReport {
    let mut report = ValidationReport::new();

    if canvas.node_count() == 0 {
        report.push(Diagnostic::warning("canvas has no nodes"));
        return report;
    }

    // Check for isolated nodes.
    for node in canvas.all_nodes() {
        let has_out = edges.edges_from(&node.id).len() > 0;
        let has_in = edges.edges_to(&node.id).len() > 0;
        if !has_out && !has_in && canvas.node_count() > 1 {
            report.push(
                Diagnostic::warning(format!("node '{}' ({}) is isolated", node.id, node.label))
                    .with_target(node.id.0.clone()),
            );
        }
    }

    report
}

#[cfg(test)]
mod unit {
    use super::*;
    use crate::import::{GraphSpec, SpecEdge, SpecNode};
    use std::collections::HashMap;

    fn minimal_spec() -> GraphSpec {
        let mut spec = GraphSpec::new("test");
        spec.nodes.push(SpecNode {
            id: "n1".into(),
            kind: "agent.llm".into(),
            label: "A".into(),
            x: 0.0,
            y: 0.0,
            config: HashMap::new(),
        });
        spec.nodes.push(SpecNode {
            id: "n2".into(),
            kind: "verifier.json_schema".into(),
            label: "V".into(),
            x: 100.0,
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
    fn valid_spec_passes() {
        let spec = minimal_spec();
        let report = validate_spec(&spec);
        assert!(report.is_valid(), "{:?}", report.diagnostics);
    }

    #[test]
    fn empty_name_fails() {
        let mut spec = minimal_spec();
        spec.name = "  ".into();
        let report = validate_spec(&spec);
        assert!(report.has_errors());
    }

    #[test]
    fn duplicate_node_id_fails() {
        let mut spec = minimal_spec();
        spec.nodes.push(SpecNode {
            id: "n1".into(),
            kind: "agent.llm".into(),
            label: "dup".into(),
            x: 0.0,
            y: 0.0,
            config: HashMap::new(),
        });
        let report = validate_spec(&spec);
        assert!(report.has_errors());
    }

    #[test]
    fn unknown_edge_target_fails() {
        let mut spec = minimal_spec();
        spec.edges.push(SpecEdge {
            id: "e_bad".into(),
            source: "n1".into(),
            target: "ghost".into(),
            edge_type: "data_flow".into(),
            label: None,
        });
        let report = validate_spec(&spec);
        assert!(report.has_errors());
    }

    #[test]
    fn self_loop_fails() {
        let mut spec = minimal_spec();
        spec.edges.push(SpecEdge {
            id: "e_loop".into(),
            source: "n1".into(),
            target: "n1".into(),
            edge_type: "data_flow".into(),
            label: None,
        });
        let report = validate_spec(&spec);
        assert!(report.has_errors());
    }
}
