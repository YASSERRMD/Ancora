/// test_import_renders - Test that importing a spec correctly populates the canvas.
use crate::import::{import_spec, parse_simple_spec, GraphSpec, SpecEdge, SpecNode};
use crate::scaffold::Position;
use std::collections::HashMap;

fn sample_spec() -> GraphSpec {
    let mut spec = GraphSpec::new("import_test");
    spec.nodes.push(SpecNode {
        id: "agent1".into(),
        kind: "agent.llm".into(),
        label: "LLM".into(),
        x: 10.0,
        y: 20.0,
        config: HashMap::new(),
    });
    spec.nodes.push(SpecNode {
        id: "tool1".into(),
        kind: "tool.web_search".into(),
        label: "Search".into(),
        x: 200.0,
        y: 20.0,
        config: {
            let mut m = HashMap::new();
            m.insert("max_results".into(), "5".into());
            m
        },
    });
    spec.nodes.push(SpecNode {
        id: "verif1".into(),
        kind: "verifier.toxicity".into(),
        label: "Toxicity".into(),
        x: 400.0,
        y: 20.0,
        config: HashMap::new(),
    });
    spec.edges.push(SpecEdge {
        id: "e1".into(),
        source: "agent1".into(),
        target: "tool1".into(),
        edge_type: "data_flow".into(),
        label: None,
    });
    spec.edges.push(SpecEdge {
        id: "e2".into(),
        source: "tool1".into(),
        target: "verif1".into(),
        edge_type: "data_flow".into(),
        label: None,
    });
    spec
}

#[test]
fn import_renders_all_nodes() {
    let result = import_spec(sample_spec()).unwrap();
    assert_eq!(
        result.canvas.node_count(),
        3,
        "all 3 nodes should be on the canvas"
    );
}

#[test]
fn import_renders_all_edges() {
    let result = import_spec(sample_spec()).unwrap();
    assert_eq!(
        result.edges.edge_count(),
        2,
        "all 2 edges should be present"
    );
}

#[test]
fn import_preserves_node_positions() {
    let result = import_spec(sample_spec()).unwrap();
    // Check that a specific node's position was preserved.
    let id = crate::scaffold::Id::new("agent1");
    let node = result.canvas.get_node(&id).expect("agent1 should exist");
    assert!((node.position.x - 10.0).abs() < 1e-9);
    assert!((node.position.y - 20.0).abs() < 1e-9);
}

#[test]
fn import_preserves_config() {
    let result = import_spec(sample_spec()).unwrap();
    let id = crate::scaffold::Id::new("tool1");
    let node = result.canvas.get_node(&id).unwrap();
    assert_eq!(
        node.config.get("max_results").map(String::as_str),
        Some("5")
    );
}

#[test]
fn import_via_text_format() {
    let text = "\
name text_import_test
node a agent.llm AgentA 0 0
node b verifier.hallucination Hallucination 200 0
edge e1 a b verification";
    let spec = parse_simple_spec(text).unwrap();
    let result = import_spec(spec).unwrap();
    assert_eq!(result.canvas.node_count(), 2);
    assert_eq!(result.edges.edge_count(), 1);
}

#[test]
fn import_no_warnings_for_clean_spec() {
    let result = import_spec(sample_spec()).unwrap();
    // A clean spec should have no warnings.
    assert!(
        result.warnings.is_empty(),
        "unexpected warnings: {:?}",
        result.warnings
    );
}
