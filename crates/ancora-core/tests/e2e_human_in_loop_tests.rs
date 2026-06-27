/// End-to-end human-in-the-loop tests (offline).
///
/// Simulates a run that suspends waiting for human approval, persists the
/// suspended state, then resumes and completes.
use ancora_core::{
    journal::{JournalStore, MemoryStore},
    replay::replay_events,
    run::RunStatus,
    suspend::{RunOutcome, SuspendedRun},
};
use ancora_proto::ancora::{
    journal_event::Event, ActivityRecordedEvent, JournalEvent, RunCompletedEvent, RunStartedEvent,
    NodeEnteredEvent, NodeExitedEvent,
};

fn ev(seq: u64, run_id: &str, event: Event) -> JournalEvent {
    JournalEvent {
        event_id: format!("{}-{}", run_id, seq),
        run_id: run_id.to_owned(),
        seq,
        recorded_at_ns: (seq * 1_000_000) as i64,
        event: Some(event),
    }
}

fn build_pre_suspend_journal(run_id: &str) -> Vec<JournalEvent> {
    vec![
        ev(0, run_id, Event::RunStarted(RunStartedEvent {
            run_id: run_id.to_owned(),
            spec_bytes: vec![],
            spec_type: "AgentSpec".into(),
        })),
        ev(1, run_id, Event::NodeEntered(NodeEnteredEvent {
            node_id: "draft-node".into(),
            node_kind: "agent".into(),
        })),
        ev(2, run_id, Event::ActivityRecorded(ActivityRecordedEvent {
            activity_key: "draft-llm".into(),
            activity_kind: "llm".into(),
            input_json: "{}".into(),
            result_json: r#"{"draft":"Please approve this plan."}"#.into(),
            replayed: false,
        })),
        ev(3, run_id, Event::NodeExited(NodeExitedEvent {
            node_id: "draft-node".into(),
            success: true,
        })),
    ]
}

fn build_post_resume_journal(run_id: &str) -> Vec<JournalEvent> {
    let mut events = build_pre_suspend_journal(run_id);
    let base = events.len() as u64;
    events.push(ev(base, run_id, Event::NodeEntered(NodeEnteredEvent {
        node_id: "await-human".into(),
        node_kind: "human".into(),
    })));
    events.push(ev(base + 1, run_id, Event::ActivityRecorded(ActivityRecordedEvent {
        activity_key: "human-approval".into(),
        activity_kind: "human".into(),
        input_json: r#"{"prompt":"approve?"}"#.into(),
        result_json: r#"{"approved":true}"#.into(),
        replayed: false,
    })));
    events.push(ev(base + 2, run_id, Event::NodeExited(NodeExitedEvent {
        node_id: "await-human".into(),
        success: true,
    })));
    events.push(ev(base + 3, run_id, Event::RunCompleted(RunCompletedEvent {
        output_json: r#"{"result":"approved-and-done"}"#.into(),
    })));
    events
}

#[test]
fn pre_suspend_journal_replays_without_completed_status() {
    let run_id = "e2e-hil-pre";
    let events = build_pre_suspend_journal(run_id);
    let state = replay_events(run_id, &events).unwrap();
    assert_ne!(state.run.status, RunStatus::Completed, "must not be Completed before resume");
}

#[test]
fn suspended_run_serializes_and_deserializes() {
    let sr = SuspendedRun {
        run_id: "hil-run-1".into(),
        node_id: "await-human".into(),
        pending_input: "Please approve this plan.".into(),
        deadline_ms: Some(1_700_000_000_000),
    };

    let json = sr.to_json().unwrap();
    let restored = SuspendedRun::from_json(&json).unwrap();

    assert_eq!(restored.run_id, "hil-run-1");
    assert_eq!(restored.node_id, "await-human");
    assert_eq!(restored.pending_input, "Please approve this plan.");
    assert_eq!(restored.deadline_ms, Some(1_700_000_000_000));
}

#[test]
fn suspended_run_without_deadline_round_trips() {
    let sr = SuspendedRun {
        run_id: "hil-run-2".into(),
        node_id: "await-human".into(),
        pending_input: "confirm?".into(),
        deadline_ms: None,
    };

    let json = sr.to_json().unwrap();
    let restored = SuspendedRun::from_json(&json).unwrap();
    assert_eq!(restored.deadline_ms, None);
}

#[test]
fn run_outcome_suspended_wraps_state() {
    let sr = SuspendedRun {
        run_id: "r".into(),
        node_id: "n".into(),
        pending_input: "approve?".into(),
        deadline_ms: None,
    };
    let outcome = RunOutcome::Suspended(sr);
    assert!(matches!(outcome, RunOutcome::Suspended(_)));
}

#[test]
fn run_outcome_completed_wraps_output() {
    let outcome = RunOutcome::Completed(r#"{"result":"done"}"#.into());
    if let RunOutcome::Completed(out) = outcome {
        assert!(out.contains("done"));
    } else {
        panic!("expected Completed");
    }
}

#[test]
fn post_resume_journal_replays_to_completed() {
    let run_id = "e2e-hil-post";
    let events = build_post_resume_journal(run_id);
    let state = replay_events(run_id, &events).unwrap();
    assert_eq!(state.run.status, RunStatus::Completed);
}

#[test]
fn resume_journal_has_human_approval_activity_key() {
    let run_id = "e2e-hil-keys";
    let events = build_post_resume_journal(run_id);
    let state = replay_events(run_id, &events).unwrap();
    assert!(state.activity_keys.contains(&"human-approval".to_string()));
}

#[test]
fn resume_journal_activity_keys_are_in_order() {
    let run_id = "e2e-hil-order";
    let events = build_post_resume_journal(run_id);
    let state = replay_events(run_id, &events).unwrap();
    assert_eq!(state.activity_keys[0], "draft-llm");
    assert_eq!(state.activity_keys[1], "human-approval");
}

#[test]
fn suspended_run_is_durable_across_store_clone() {
    let store = MemoryStore::new();
    let run_id = "e2e-hil-durable";

    for ev in build_pre_suspend_journal(run_id) {
        store.append(run_id, ev).unwrap();
    }

    let store2 = store.clone();
    let events = store2.read(run_id).unwrap();
    assert_eq!(events.len(), 4, "pre-suspend journal must survive store clone");
}

#[test]
fn two_pending_inputs_round_trip_correctly() {
    let inputs = ["Please approve.", "Confirm action."];
    for (i, input) in inputs.iter().enumerate() {
        let sr = SuspendedRun {
            run_id: format!("r-{}", i),
            node_id: "n".into(),
            pending_input: (*input).to_owned(),
            deadline_ms: None,
        };
        let restored = SuspendedRun::from_json(&sr.to_json().unwrap()).unwrap();
        assert_eq!(&restored.pending_input, input);
    }
}
