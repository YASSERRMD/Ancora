/// test_trace_overlay - Test that the trace overlay renders correctly from run results.
use crate::import::{GraphSpec, SpecEdge, SpecNode};
use crate::runner::{run_spec, LocalBackendConfig, RunResult, RunStatus, RunStep, StepStatus};
use crate::trace_overlay::{LiveTraceOverlay, OverlayStatus, TraceOverlay};
use std::collections::HashMap;

fn make_completed_result() -> RunResult {
    RunResult {
        run_id: "run_overlay_test".into(),
        spec_name: "overlay_graph".into(),
        status: RunStatus::Completed,
        steps: vec![
            RunStep {
                step_index: 0,
                node_id: "n1".into(),
                node_kind: "agent.llm".into(),
                status: StepStatus::Succeeded,
                output: Some("agent output".into()),
                error: None,
                duration_ms: 15,
            },
            RunStep {
                step_index: 1,
                node_id: "n2".into(),
                node_kind: "verifier.hallucination".into(),
                status: StepStatus::Succeeded,
                output: Some("verified".into()),
                error: None,
                duration_ms: 5,
            },
        ],
        total_duration_ms: 20,
        outputs: HashMap::new(),
    }
}

fn make_failed_result() -> RunResult {
    RunResult {
        run_id: "run_fail".into(),
        spec_name: "fail_graph".into(),
        status: RunStatus::Failed,
        steps: vec![
            RunStep {
                step_index: 0,
                node_id: "x".into(),
                node_kind: "agent.llm".into(),
                status: StepStatus::Succeeded,
                output: Some("ok".into()),
                error: None,
                duration_ms: 10,
            },
            RunStep {
                step_index: 1,
                node_id: "y".into(),
                node_kind: "verifier.json_schema".into(),
                status: StepStatus::Failed,
                output: None,
                error: Some("schema mismatch".into()),
                duration_ms: 3,
            },
        ],
        total_duration_ms: 13,
        outputs: HashMap::new(),
    }
}

#[test]
fn overlay_renders_all_nodes() {
    let result = make_completed_result();
    let overlay = TraceOverlay::from_run_result(&result);
    assert_eq!(overlay.node_overlays.len(), 2);
}

#[test]
fn overlay_succeeded_status_correct() {
    let result = make_completed_result();
    let overlay = TraceOverlay::from_run_result(&result);
    let n1 = overlay.node_overlay("n1").unwrap();
    assert_eq!(n1.status, OverlayStatus::Succeeded);
}

#[test]
fn overlay_failed_status_correct() {
    let result = make_failed_result();
    let overlay = TraceOverlay::from_run_result(&result);
    let y = overlay.node_overlay("y").unwrap();
    assert_eq!(y.status, OverlayStatus::Failed);
}

#[test]
fn overlay_duration_label_formatted() {
    let result = make_completed_result();
    let overlay = TraceOverlay::from_run_result(&result);
    let n1 = overlay.node_overlay("n1").unwrap();
    assert!(n1.duration_label.contains("ms"));
}

#[test]
fn overlay_output_excerpt_present() {
    let result = make_completed_result();
    let overlay = TraceOverlay::from_run_result(&result);
    let n1 = overlay.node_overlay("n1").unwrap();
    assert!(n1.output_excerpt.is_some());
}

#[test]
fn overlay_error_message_on_failed_node() {
    let result = make_failed_result();
    let overlay = TraceOverlay::from_run_result(&result);
    let y = overlay.node_overlay("y").unwrap();
    assert!(y.error_message.is_some());
    assert!(y
        .error_message
        .as_ref()
        .unwrap()
        .contains("schema mismatch"));
}

#[test]
fn overlay_clear_resets_state() {
    let result = make_completed_result();
    let mut overlay = TraceOverlay::from_run_result(&result);
    overlay.clear();
    assert!(!overlay.is_active());
}

#[test]
fn overlay_from_run_result_via_runner() {
    let mut spec = GraphSpec::new("live_overlay");
    spec.nodes.push(SpecNode {
        id: "a".into(),
        kind: "agent.llm".into(),
        label: "A".into(),
        x: 0.0,
        y: 0.0,
        config: HashMap::new(),
    });
    spec.nodes.push(SpecNode {
        id: "b".into(),
        kind: "verifier.toxicity".into(),
        label: "B".into(),
        x: 100.0,
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

    let config = LocalBackendConfig::default();
    let result = run_spec(&spec, &config).unwrap();
    let overlay = TraceOverlay::from_run_result(&result);

    assert!(overlay.is_active());
    assert_eq!(overlay.node_overlays.len(), 2);
    let succeeded = overlay.succeeded_nodes();
    assert_eq!(succeeded.len(), 2);
}

#[test]
fn live_overlay_apply_step() {
    let step = RunStep {
        step_index: 0,
        node_id: "live_n".into(),
        node_kind: "agent.llm".into(),
        status: StepStatus::Succeeded,
        output: Some("live output".into()),
        error: None,
        duration_ms: 7,
    };
    let mut live = LiveTraceOverlay::new();
    live.apply_step(&step);
    let ov = live.inner.node_overlay("live_n").unwrap();
    assert_eq!(ov.status, OverlayStatus::Succeeded);
    assert_eq!(live.current_step_index, Some(0));
}
