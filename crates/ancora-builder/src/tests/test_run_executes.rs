/// test_run_executes - Test that a graph can be run from the builder against the local backend.

use crate::import::{GraphSpec, SpecEdge, SpecNode};
use crate::runner::{run_spec, LocalBackendConfig, RunStatus, StepStatus};
use std::collections::HashMap;

fn minimal_spec(name: &str) -> GraphSpec {
    let mut spec = GraphSpec::new(name);
    spec.nodes.push(SpecNode {
        id: "a".into(),
        kind: "agent.llm".into(),
        label: "Agent".into(),
        x: 0.0,
        y: 0.0,
        config: HashMap::new(),
    });
    spec.nodes.push(SpecNode {
        id: "b".into(),
        kind: "verifier.toxicity".into(),
        label: "Verifier".into(),
        x: 200.0,
        y: 0.0,
        config: HashMap::new(),
    });
    spec.edges.push(SpecEdge {
        id: "e1".into(),
        source: "a".into(),
        target: "b".into(),
        edge_type: "data_flow".into(),
        label: None,
    });
    spec
}

#[test]
fn run_executes_all_steps() {
    let spec = minimal_spec("run_all_steps");
    let config = LocalBackendConfig::default();
    let result = run_spec(&spec, &config).unwrap();
    assert_eq!(result.steps.len(), 2);
}

#[test]
fn run_completes_successfully() {
    let spec = minimal_spec("run_success");
    let config = LocalBackendConfig::default();
    let result = run_spec(&spec, &config).unwrap();
    assert_eq!(result.status, RunStatus::Completed);
}

#[test]
fn run_steps_have_succeeded_status() {
    let spec = minimal_spec("run_step_status");
    let config = LocalBackendConfig::default();
    let result = run_spec(&spec, &config).unwrap();
    for step in &result.steps {
        assert_eq!(step.status, StepStatus::Succeeded, "step {} should have succeeded", step.node_id);
    }
}

#[test]
fn run_produces_outputs_per_node() {
    let spec = minimal_spec("run_outputs");
    let config = LocalBackendConfig::default();
    let result = run_spec(&spec, &config).unwrap();
    assert!(result.outputs.contains_key("a"), "output for node a missing");
    assert!(result.outputs.contains_key("b"), "output for node b missing");
}

#[test]
fn run_invalid_spec_returns_validation_error() {
    let spec = GraphSpec::new(""); // invalid: empty name
    let config = LocalBackendConfig::default();
    let err = run_spec(&spec, &config).unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("validation"), "expected validation error, got: {}", msg);
}

#[test]
fn run_returns_run_id() {
    let spec = minimal_spec("run_id_test");
    let config = LocalBackendConfig::default();
    let result = run_spec(&spec, &config).unwrap();
    assert!(!result.run_id.is_empty());
    assert!(result.run_id.starts_with("run_"));
}

#[test]
fn run_topological_order_respected() {
    // n1 -> n2 -> n3: step_index for n1 < n2 < n3.
    let mut spec = GraphSpec::new("topo_order");
    spec.nodes.push(SpecNode { id: "n1".into(), kind: "agent.llm".into(), label: "First".into(), x: 0.0, y: 0.0, config: HashMap::new() });
    spec.nodes.push(SpecNode { id: "n2".into(), kind: "tool.web_search".into(), label: "Second".into(), x: 100.0, y: 0.0, config: HashMap::new() });
    spec.nodes.push(SpecNode { id: "n3".into(), kind: "verifier.toxicity".into(), label: "Third".into(), x: 200.0, y: 0.0, config: HashMap::new() });
    spec.edges.push(SpecEdge { id: "e1".into(), source: "n1".into(), target: "n2".into(), edge_type: "data_flow".into(), label: None });
    spec.edges.push(SpecEdge { id: "e2".into(), source: "n2".into(), target: "n3".into(), edge_type: "data_flow".into(), label: None });

    let config = LocalBackendConfig::default();
    let result = run_spec(&spec, &config).unwrap();

    let idx = |id: &str| result.steps.iter().position(|s| s.node_id == id).unwrap();
    assert!(idx("n1") < idx("n2"));
    assert!(idx("n2") < idx("n3"));
}
