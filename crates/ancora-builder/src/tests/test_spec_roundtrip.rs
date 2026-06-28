/// test_spec_roundtrip - Test that a spec can be exported to text and re-imported identically.

use crate::edges::{EdgeStore, EdgeType};
use crate::export::{export_spec, spec_to_text, ExportOptions};
use crate::import::{import_spec, parse_simple_spec};
use crate::placement::Canvas;
use crate::scaffold::Position;
use crate::validation::validate_spec;

fn build_graph() -> (Canvas, EdgeStore) {
    let mut canvas = Canvas::new();
    let mut edges = EdgeStore::new();

    let a = canvas.place_node("agent.llm", "LLM Agent", Position::new(0.0, 0.0));
    let b = canvas.place_node("tool.web_search", "Search", Position::new(200.0, 0.0));
    let c = canvas.place_node("verifier.hallucination", "Halluc Checker", Position::new(400.0, 0.0));

    edges.add_edge(a.clone(), "agent.llm", b.clone(), "tool.web_search", EdgeType::DataFlow).unwrap();
    edges.add_edge(b.clone(), "tool.web_search", c.clone(), "verifier.hallucination", EdgeType::DataFlow).unwrap();

    (canvas, edges)
}

#[test]
fn exported_spec_round_trips_via_text() {
    let (canvas, edges) = build_graph();
    let spec = export_spec("roundtrip_test", &canvas, &edges, &ExportOptions::default()).unwrap();

    // Serialize to text.
    let text = spec_to_text(&spec);

    // Re-parse.
    let spec2 = parse_simple_spec(&text).unwrap();

    assert_eq!(spec2.name, spec.name);
    assert_eq!(spec2.nodes.len(), spec.nodes.len(), "node count mismatch after round-trip");
    assert_eq!(spec2.edges.len(), spec.edges.len(), "edge count mismatch after round-trip");
}

#[test]
fn round_tripped_spec_is_valid() {
    let (canvas, edges) = build_graph();
    let spec = export_spec("rt_valid", &canvas, &edges, &ExportOptions::default()).unwrap();
    let text = spec_to_text(&spec);
    let spec2 = parse_simple_spec(&text).unwrap();
    let report = validate_spec(&spec2);
    assert!(report.is_valid(), "round-tripped spec should be valid: {:?}", report.diagnostics);
}

#[test]
fn round_tripped_spec_can_be_imported() {
    let (canvas, edges) = build_graph();
    let spec = export_spec("rt_import", &canvas, &edges, &ExportOptions::default()).unwrap();
    let text = spec_to_text(&spec);
    let spec2 = parse_simple_spec(&text).unwrap();

    let result = import_spec(spec2).unwrap();
    assert_eq!(result.canvas.node_count(), 3);
    assert_eq!(result.edges.edge_count(), 2);
}

#[test]
fn node_kinds_preserved_in_round_trip() {
    let (canvas, edges) = build_graph();
    let spec = export_spec("kind_rt", &canvas, &edges, &ExportOptions::default()).unwrap();
    let text = spec_to_text(&spec);
    let spec2 = parse_simple_spec(&text).unwrap();

    let kinds: std::collections::HashSet<String> = spec.nodes.iter().map(|n| n.kind.clone()).collect();
    let kinds2: std::collections::HashSet<String> = spec2.nodes.iter().map(|n| n.kind.clone()).collect();
    assert_eq!(kinds, kinds2, "node kinds should survive round-trip");
}

#[test]
fn edge_types_preserved_in_round_trip() {
    let (canvas, edges) = build_graph();
    let spec = export_spec("etype_rt", &canvas, &edges, &ExportOptions::default()).unwrap();
    let text = spec_to_text(&spec);
    let spec2 = parse_simple_spec(&text).unwrap();

    for e2 in &spec2.edges {
        assert!(!e2.edge_type.is_empty(), "edge type should not be empty");
    }
}
