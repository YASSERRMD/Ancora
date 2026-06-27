use ancora_core::suspend::{RunOutcome, SuspendedRun};

fn make_suspended(run_id: &str, node_id: &str, input: &str, deadline: Option<u64>) -> SuspendedRun {
    SuspendedRun {
        run_id: run_id.to_string(),
        node_id: node_id.to_string(),
        pending_input: input.to_string(),
        deadline_ms: deadline,
    }
}

#[test]
fn suspended_run_round_trips_through_json_without_deadline() {
    let sr = make_suspended("run-42", "review-node", "Please approve.", None);
    let json = sr.to_json().unwrap();
    let restored = SuspendedRun::from_json(&json).unwrap();

    assert_eq!(restored.run_id, "run-42");
    assert_eq!(restored.node_id, "review-node");
    assert_eq!(restored.pending_input, "Please approve.");
    assert_eq!(restored.deadline_ms, None);
}

#[test]
fn suspended_run_round_trips_with_deadline() {
    let sr = make_suspended("run-43", "approve", "Approve?", Some(1_700_000_000_000));
    let json = sr.to_json().unwrap();
    let restored = SuspendedRun::from_json(&json).unwrap();

    assert_eq!(restored.deadline_ms, Some(1_700_000_000_000));
}

#[test]
fn from_json_fails_on_missing_run_id() {
    let bad = r#"{"node_id":"n","pending_input":"x","deadline_ms":null}"#;
    assert!(
        SuspendedRun::from_json(bad).is_err(),
        "must fail when run_id is absent"
    );
}

#[test]
fn from_json_fails_on_missing_node_id() {
    let bad = r#"{"run_id":"r","pending_input":"x","deadline_ms":null}"#;
    assert!(
        SuspendedRun::from_json(bad).is_err(),
        "must fail when node_id is absent"
    );
}

#[test]
fn from_json_fails_on_missing_pending_input() {
    let bad = r#"{"run_id":"r","node_id":"n","deadline_ms":null}"#;
    assert!(
        SuspendedRun::from_json(bad).is_err(),
        "must fail when pending_input is absent"
    );
}

#[test]
fn from_json_fails_on_empty_string() {
    assert!(SuspendedRun::from_json("").is_err(), "empty string must be rejected");
}

#[test]
fn from_json_fails_on_malformed_json() {
    assert!(SuspendedRun::from_json("{bad json}").is_err(), "malformed JSON must be rejected");
}

#[test]
fn run_outcome_completed_variant_holds_output() {
    let outcome = RunOutcome::Completed("the final answer".to_string());
    if let RunOutcome::Completed(output) = outcome {
        assert_eq!(output, "the final answer");
    } else {
        panic!("expected Completed variant");
    }
}

#[test]
fn run_outcome_suspended_variant_holds_state() {
    let sr = make_suspended("run-s", "await-node", "confirm?", Some(9999));
    let outcome = RunOutcome::Suspended(sr);
    if let RunOutcome::Suspended(s) = outcome {
        assert_eq!(s.run_id, "run-s");
        assert_eq!(s.deadline_ms, Some(9999));
    } else {
        panic!("expected Suspended variant");
    }
}

#[test]
fn json_round_trip_preserves_unicode_in_pending_input() {
    let sr = make_suspended("run-u", "node-u", "Approve? \u{1F680}", None);
    let json = sr.to_json().unwrap();
    let restored = SuspendedRun::from_json(&json).unwrap();
    assert_eq!(restored.pending_input, "Approve? \u{1F680}");
}
