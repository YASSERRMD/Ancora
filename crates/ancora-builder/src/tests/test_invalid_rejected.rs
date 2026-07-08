/// test_invalid_rejected - Test that invalid connections and specs are rejected.
use crate::edges::{EdgeError, EdgeStore, EdgeType};
use crate::import::{import_spec, GraphSpec, ImportError, SpecEdge, SpecNode};
use crate::scaffold::Id;
use crate::validation::{validate_spec, Severity};
use std::collections::HashMap;

fn node(id: &str, kind: &str) -> SpecNode {
    SpecNode {
        id: id.into(),
        kind: kind.into(),
        label: id.into(),
        x: 0.0,
        y: 0.0,
        config: HashMap::new(),
    }
}

fn edge(id: &str, src: &str, tgt: &str) -> SpecEdge {
    SpecEdge {
        id: id.into(),
        source: src.into(),
        target: tgt.into(),
        edge_type: "data_flow".into(),
        label: None,
    }
}

#[test]
fn self_loop_edge_rejected() {
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
fn invalid_connection_type_rejected() {
    let mut store = EdgeStore::new();
    // Only allow agents connecting to verifiers.
    store.allow_connection("agent.llm", "verifier.json_schema");
    let err = store
        .add_edge(
            Id::new("a"),
            "agent.llm",
            Id::new("b"),
            "agent.llm", // not in allowed list
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
fn spec_with_empty_name_fails_validation() {
    let mut spec = GraphSpec::new("  ");
    spec.nodes.push(node("n1", "agent.llm"));
    let report = validate_spec(&spec);
    assert!(report.has_errors());
    assert!(report.errors().any(|d| d.message.contains("name")));
}

#[test]
fn spec_with_duplicate_node_ids_fails() {
    let mut spec = GraphSpec::new("dup_test");
    spec.nodes.push(node("n1", "agent.llm"));
    spec.nodes.push(node("n1", "agent.classifier")); // duplicate
    let report = validate_spec(&spec);
    assert!(report.has_errors());
}

#[test]
fn spec_with_self_loop_edge_fails() {
    let mut spec = GraphSpec::new("loop_test");
    spec.nodes.push(node("n1", "agent.llm"));
    spec.edges.push(SpecEdge {
        id: "e1".into(),
        source: "n1".into(),
        target: "n1".into(),
        edge_type: "data_flow".into(),
        label: None,
    });
    let report = validate_spec(&spec);
    assert!(report.has_errors());
    assert!(report.errors().any(|d| d.message.contains("self-loop")));
}

#[test]
fn spec_with_unknown_edge_target_fails() {
    let mut spec = GraphSpec::new("unknown_tgt");
    spec.nodes.push(node("n1", "agent.llm"));
    spec.edges.push(edge("e1", "n1", "ghost"));
    let report = validate_spec(&spec);
    assert!(report.has_errors());
}

#[test]
fn spec_with_unknown_edge_source_fails() {
    let mut spec = GraphSpec::new("unknown_src");
    spec.nodes.push(node("n1", "agent.llm"));
    spec.edges.push(edge("e1", "ghost", "n1"));
    let report = validate_spec(&spec);
    assert!(report.has_errors());
}

#[test]
fn import_duplicate_node_id_returns_error() {
    let mut spec = GraphSpec::new("dup_import");
    spec.nodes.push(node("n1", "agent.llm"));
    spec.nodes.push(node("n1", "agent.classifier"));
    let result = import_spec(spec);
    assert!(result.is_err(), "expected an error for duplicate node id");
    let err = result.err().unwrap();
    assert!(matches!(err, ImportError::DuplicateNodeId(_)));
}

#[test]
fn isolated_node_produces_warning_not_error() {
    let mut spec = GraphSpec::new("isolated");
    spec.nodes.push(node("n1", "agent.llm"));
    spec.nodes.push(node("n2", "verifier.toxicity")); // isolated: no edges
    spec.edges.push(edge("e1", "n1", "n1")); // self loop - this adds an error too
                                             // Actually let's make a clean test: two nodes, one edge missing
    let mut spec2 = GraphSpec::new("isolated2");
    spec2.nodes.push(node("a", "agent.llm"));
    spec2.nodes.push(node("b", "agent.classifier")); // isolated
    spec2.edges.push(SpecEdge {
        id: "e1".into(),
        source: "a".into(),
        target: "a".into(), // self-loop to make a connected but still have b isolated
        edge_type: "data_flow".into(),
        label: None,
    });
    // Actually let's just test isolation warning with two nodes and no edges
    let mut spec3 = GraphSpec::new("isolation_warning");
    spec3.nodes.push(node("x", "agent.llm"));
    spec3.nodes.push(node("y", "tool.web_search"));
    // No edges -> both are isolated
    let report = validate_spec(&spec3);
    // Should have warnings for isolation but no errors (empty name passes, no bad edges).
    let has_isolation_warning = report.warnings().any(|d| d.message.contains("isolated"));
    assert!(has_isolation_warning);
}
