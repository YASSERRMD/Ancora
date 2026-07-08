/// test_offline_backend - Test that the runner works fully offline without any network calls.
use crate::import::{GraphSpec, SpecEdge, SpecNode};
use crate::runner::{run_spec, LocalBackendConfig, RunStatus};
use std::collections::HashMap;

fn make_spec(name: &str, nodes: &[(&str, &str)], edges: &[(&str, &str, &str)]) -> GraphSpec {
    let mut spec = GraphSpec::new(name);
    for (i, (id, kind)) in nodes.iter().enumerate() {
        spec.nodes.push(SpecNode {
            id: id.to_string(),
            kind: kind.to_string(),
            label: format!("Node {}", i),
            x: (i as f64) * 100.0,
            y: 0.0,
            config: HashMap::new(),
        });
    }
    for (i, (src, tgt, etype)) in edges.iter().enumerate() {
        spec.edges.push(SpecEdge {
            id: format!("e{}", i),
            source: src.to_string(),
            target: tgt.to_string(),
            edge_type: etype.to_string(),
            label: None,
        });
    }
    spec
}

#[test]
fn offline_run_completes() {
    let spec = make_spec(
        "offline_test",
        &[("a", "agent.llm"), ("b", "verifier.toxicity")],
        &[("a", "b", "data_flow")],
    );
    let config = LocalBackendConfig {
        offline: true,
        ..Default::default()
    };
    let result = run_spec(&spec, &config).expect("offline run should succeed");
    assert_eq!(result.status, RunStatus::Completed);
}

#[test]
fn offline_run_no_network_url_required() {
    // In offline mode, the base_url is irrelevant and should not cause failures.
    let spec = make_spec("offline_no_url", &[("x", "agent.classifier")], &[]);
    let config = LocalBackendConfig {
        offline: true,
        base_url: "http://offline-does-not-exist.local:9999".into(),
        ..Default::default()
    };
    let result = run_spec(&spec, &config).expect("offline run should ignore url");
    assert_eq!(result.status, RunStatus::Completed);
}

#[test]
fn offline_multi_node_pipeline() {
    let spec = make_spec(
        "offline_pipeline",
        &[
            ("src", "control.router"),
            ("llm", "agent.llm"),
            ("search", "tool.web_search"),
            ("verify", "verifier.hallucination"),
            ("sink", "control.merge"),
        ],
        &[
            ("src", "llm", "data_flow"),
            ("llm", "search", "data_flow"),
            ("search", "verify", "data_flow"),
            ("verify", "sink", "data_flow"),
        ],
    );
    let config = LocalBackendConfig {
        offline: true,
        ..Default::default()
    };
    let result = run_spec(&spec, &config).unwrap();
    assert_eq!(result.status, RunStatus::Completed);
    assert_eq!(result.steps.len(), 5);
}

#[test]
fn offline_all_steps_produce_output() {
    let spec = make_spec(
        "offline_outputs",
        &[("a", "agent.llm"), ("b", "tool.file_reader")],
        &[("a", "b", "data_flow")],
    );
    let config = LocalBackendConfig {
        offline: true,
        ..Default::default()
    };
    let result = run_spec(&spec, &config).unwrap();
    for step in &result.steps {
        assert!(
            step.output.is_some(),
            "step {} should have output",
            step.node_id
        );
    }
}

#[test]
fn offline_run_duration_tracked() {
    let spec = make_spec(
        "offline_duration",
        &[("a", "agent.llm"), ("b", "verifier.json_schema")],
        &[("a", "b", "data_flow")],
    );
    let config = LocalBackendConfig {
        offline: true,
        ..Default::default()
    };
    let result = run_spec(&spec, &config).unwrap();
    // Total duration should be sum of step durations.
    let step_total: u64 = result.steps.iter().map(|s| s.duration_ms).sum();
    assert_eq!(result.total_duration_ms, step_total);
}
