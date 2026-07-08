use ancora_core::journal_mask::{assert_structurally_equal, mask_events};
use ancora_proto::ancora::{
    journal_event::Event, ActivityRecordedEvent, JournalEvent, NodeEnteredEvent, NodeExitedEvent,
    RunCompletedEvent, RunStartedEvent,
};

fn ev(seq: u64, run: &str, inner: Event) -> JournalEvent {
    JournalEvent {
        event_id: format!("{}-{}", run, seq),
        run_id: run.to_owned(),
        seq,
        recorded_at_ns: 0,
        event: Some(inner),
    }
}

fn started(seq: u64, run: &str) -> JournalEvent {
    ev(
        seq,
        run,
        Event::RunStarted(RunStartedEvent {
            run_id: run.to_owned(),
            spec_bytes: vec![],
            spec_type: "AgentSpec".into(),
        }),
    )
}

fn activity(seq: u64, run: &str, key: &str) -> JournalEvent {
    ev(
        seq,
        run,
        Event::ActivityRecorded(ActivityRecordedEvent {
            activity_key: key.to_owned(),
            activity_kind: "llm_call".to_owned(),
            input_json: "{}".to_owned(),
            result_json: "secret model text".to_owned(),
            replayed: false,
        }),
    )
}

fn node_entered(seq: u64, run: &str, node: &str) -> JournalEvent {
    ev(
        seq,
        run,
        Event::NodeEntered(NodeEnteredEvent {
            node_id: node.to_owned(),
            node_kind: "agent".to_owned(),
        }),
    )
}

fn node_exited(seq: u64, run: &str, node: &str) -> JournalEvent {
    ev(
        seq,
        run,
        Event::NodeExited(NodeExitedEvent {
            node_id: node.to_owned(),
            success: true,
        }),
    )
}

fn completed(seq: u64, run: &str) -> JournalEvent {
    ev(
        seq,
        run,
        Event::RunCompleted(RunCompletedEvent {
            output_json: "secret output".to_owned(),
        }),
    )
}

#[test]
fn mask_activity_result_is_not_exposed() {
    let events = vec![activity(0, "r", "k")];
    let masked = mask_events(&events);
    assert_eq!(masked.len(), 1);
    // The masked event does NOT contain model-generated text
    let repr = format!("{:?}", masked[0]);
    assert!(
        !repr.contains("secret"),
        "model-generated content must not appear in MaskedEvent"
    );
}

#[test]
fn mask_node_entered_preserves_node_id() {
    let events = vec![node_entered(0, "r", "my-node")];
    let masked = mask_events(&events);
    assert_eq!(masked[0].node_id, Some("my-node".into()));
}

#[test]
fn mask_node_exited_preserves_node_id() {
    let events = vec![node_exited(0, "r", "exit-node")];
    let masked = mask_events(&events);
    assert_eq!(masked[0].node_id, Some("exit-node".into()));
}

#[test]
fn mask_empty_slice_returns_empty() {
    assert!(mask_events(&[]).is_empty());
}

#[test]
fn two_full_traces_with_same_shape_pass_structural_equality() {
    let trace = |run: &str| {
        mask_events(&[
            started(0, run),
            node_entered(1, run, "a"),
            activity(2, run, "act"),
            node_exited(3, run, "a"),
            completed(4, run),
        ])
    };
    assert_structurally_equal(&trace("run-1"), &trace("run-2")).unwrap();
}

#[test]
fn different_node_id_in_same_position_fails_equality() {
    let a = mask_events(&[node_entered(0, "r1", "node-a"), completed(1, "r1")]);
    let b = mask_events(&[node_entered(0, "r2", "node-b"), completed(1, "r2")]);
    assert!(assert_structurally_equal(&a, &b).is_err());
}

#[test]
fn loop_iteration_same_node_multiple_times_structurally_equal() {
    let loop_trace = |run: &str| {
        mask_events(&[
            started(0, run),
            node_entered(1, run, "loop-body"),
            node_exited(2, run, "loop-body"),
            node_entered(3, run, "loop-body"),
            node_exited(4, run, "loop-body"),
            completed(5, run),
        ])
    };
    assert_structurally_equal(&loop_trace("run-1"), &loop_trace("run-2")).unwrap();
}

#[test]
fn sequence_number_not_compared_in_structural_check() {
    // Two traces with same kinds/node_ids but different seq numbers
    let a = mask_events(&[started(0, "r1"), completed(100, "r1")]);
    let b = mask_events(&[started(99, "r2"), completed(200, "r2")]);
    assert_structurally_equal(&a, &b).unwrap();
}

#[test]
fn null_inner_event_is_dropped_from_mask() {
    let mut null_ev = started(0, "r");
    null_ev.event = None;
    let events = vec![null_ev, started(1, "r"), completed(2, "r")];
    let masked = mask_events(&events);
    assert_eq!(masked.len(), 2, "null inner event must be silently skipped");
}
