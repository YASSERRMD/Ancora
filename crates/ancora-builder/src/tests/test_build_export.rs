/// test_build_export - Test that a graph can be built on the canvas and exported to a valid spec.
use crate::edges::{EdgeStore, EdgeType};
use crate::export::{export_spec, ExportOptions};
use crate::placement::Canvas;
use crate::scaffold::Position;
use crate::validation::validate_spec;

#[test]
fn build_and_export_valid_graph() {
    let mut canvas = Canvas::new();
    let mut edges = EdgeStore::new();

    let a = canvas.place_node("agent.llm", "My Agent", Position::new(0.0, 0.0));
    let b = canvas.place_node(
        "verifier.json_schema",
        "Verifier",
        Position::new(300.0, 0.0),
    );

    edges
        .add_edge(
            a.clone(),
            "agent.llm",
            b.clone(),
            "verifier.json_schema",
            EdgeType::DataFlow,
        )
        .expect("edge should be accepted");

    let spec = export_spec(
        "build_export_test",
        &canvas,
        &edges,
        &ExportOptions::default(),
    )
    .expect("export should succeed");

    assert_eq!(spec.nodes.len(), 2);
    assert_eq!(spec.edges.len(), 1);

    let report = validate_spec(&spec);
    assert!(
        report.is_valid(),
        "exported spec should be valid: {:?}",
        report.diagnostics
    );
}

#[test]
fn export_preserves_node_positions() {
    let mut canvas = Canvas::new();
    let edges = EdgeStore::new();

    canvas.place_node("agent.llm", "Agent", Position::new(123.0, 456.0));

    let opts = ExportOptions {
        include_positions: true,
        ..Default::default()
    };

    let spec = export_spec("pos_test", &canvas, &edges, &opts);
    // Empty edge store - export fails because we have only 1 node but edges are empty;
    // however the function only requires nodes > 0, so this should succeed.
    let spec = spec.expect("should export single node");

    let node = &spec.nodes[0];
    assert!((node.x - 123.0).abs() < 1e-9);
    assert!((node.y - 456.0).abs() < 1e-9);
}

#[test]
fn export_multi_node_graph() {
    let mut canvas = Canvas::new();
    let mut edges = EdgeStore::new();

    let a = canvas.place_node("control.router", "Router", Position::new(0.0, 0.0));
    let b = canvas.place_node("agent.llm", "Agent A", Position::new(200.0, -80.0));
    let c = canvas.place_node("agent.llm", "Agent B", Position::new(200.0, 80.0));
    let d = canvas.place_node("control.merge", "Merge", Position::new(400.0, 0.0));

    edges
        .add_edge(
            a.clone(),
            "control.router",
            b.clone(),
            "agent.llm",
            EdgeType::DataFlow,
        )
        .unwrap();
    edges
        .add_edge(
            a.clone(),
            "control.router",
            c.clone(),
            "agent.llm",
            EdgeType::DataFlow,
        )
        .unwrap();
    edges
        .add_edge(
            b.clone(),
            "agent.llm",
            d.clone(),
            "control.merge",
            EdgeType::DataFlow,
        )
        .unwrap();
    edges
        .add_edge(
            c.clone(),
            "agent.llm",
            d.clone(),
            "control.merge",
            EdgeType::DataFlow,
        )
        .unwrap();

    let spec = export_spec("fan_out", &canvas, &edges, &ExportOptions::default()).unwrap();

    assert_eq!(spec.nodes.len(), 4);
    assert_eq!(spec.edges.len(), 4);

    let report = validate_spec(&spec);
    assert!(report.is_valid(), "{:?}", report.diagnostics);
}
