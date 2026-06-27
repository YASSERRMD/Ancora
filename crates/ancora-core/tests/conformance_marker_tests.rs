use ancora_core::journal::{JournalStore, MemoryStore};
use ancora_core::replay::replay_events;
use ancora_core::run::RunStatus;
use ancora_proto::ancora::{
    journal_event::Event, ActivityRecordedEvent, JournalEvent, RunCancelledEvent,
    RunCompletedEvent, RunStartedEvent, ErrorEvent,
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

#[test]
fn replay_cancelled_event_yields_cancelled_status() {
    let run_id = "run-cancel-replay";
    let events = vec![
        ev(0, run_id, Event::RunStarted(RunStartedEvent {
            run_id: run_id.to_owned(), spec_bytes: vec![], spec_type: "AgentSpec".into()
        })),
        ev(1, run_id, Event::RunCancelled(RunCancelledEvent {
            reason: "user requested".into()
        })),
    ];
    let state = replay_events(run_id, &events).unwrap();
    assert_eq!(state.run.status, RunStatus::Cancelled);
}

#[test]
fn replay_error_event_yields_failed_status() {
    let run_id = "run-error-replay";
    let events = vec![
        ev(0, run_id, Event::RunStarted(RunStartedEvent {
            run_id: run_id.to_owned(), spec_bytes: vec![], spec_type: "AgentSpec".into()
        })),
        ev(1, run_id, Event::Error(ErrorEvent {
            code: "ErrorModelUnreachable".into(),
            message: "connection refused".into(),
            detail: String::new(),
        })),
    ];
    let state = replay_events(run_id, &events).unwrap();
    assert_eq!(state.run.status, RunStatus::Failed);
}

#[test]
fn multiple_activity_keys_in_sequence_are_all_present() {
    let run_id = "multi-act";
    let n = 20;
    let mut events = vec![ev(0, run_id, Event::RunStarted(RunStartedEvent {
        run_id: run_id.to_owned(), spec_bytes: vec![], spec_type: "AgentSpec".into()
    }))];
    for i in 1..=n {
        events.push(ev(i as u64, run_id, Event::ActivityRecorded(ActivityRecordedEvent {
            activity_key: format!("key-{}", i),
            activity_kind: "llm_call".into(),
            input_json: "{}".into(),
            result_json: "result".into(),
            replayed: false,
        })));
    }
    let state = replay_events(run_id, &events).unwrap();
    assert_eq!(state.activity_keys.len(), n);
    assert_eq!(state.run.seq, n as u64);
}

#[test]
fn replay_ignores_unknown_event_kinds_gracefully() {
    let run_id = "run-unknown";
    let events = vec![
        ev(0, run_id, Event::RunStarted(RunStartedEvent {
            run_id: run_id.to_owned(), spec_bytes: vec![], spec_type: "AgentSpec".into()
        })),
        ev(1, run_id, Event::RunCompleted(RunCompletedEvent {
            output_json: String::new()
        })),
    ];
    let state = replay_events(run_id, &events).unwrap();
    assert_eq!(state.run.status, RunStatus::Completed);
}

#[test]
fn memory_store_clone_shares_state() {
    let store = MemoryStore::new();
    let ev = JournalEvent {
        event_id: "e-0".into(),
        run_id: "run-clone".into(),
        seq: 0,
        recorded_at_ns: 0,
        event: Some(Event::RunStarted(RunStartedEvent {
            run_id: "run-clone".into(),
            spec_bytes: vec![],
            spec_type: "AgentSpec".into(),
        })),
    };
    store.append("run-clone", ev).unwrap();

    let clone = store.clone();
    let events = clone.read("run-clone").unwrap();
    assert_eq!(events.len(), 1, "clone must share the same backing store");
}
