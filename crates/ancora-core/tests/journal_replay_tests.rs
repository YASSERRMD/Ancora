use ancora_core::journal::{JournalStore, MemoryStore};
use ancora_core::replay::replay_events;
use ancora_core::run::RunStatus;
use ancora_proto::ancora::{
    journal_event::Event, ActivityRecordedEvent, JournalEvent, RunCompletedEvent, RunStartedEvent,
};

fn started(run_id: &str) -> JournalEvent {
    JournalEvent {
        event_id: format!("{}-started", run_id),
        run_id: run_id.to_owned(),
        seq: 0,
        recorded_at_ns: 0,
        event: Some(Event::RunStarted(RunStartedEvent {
            run_id: run_id.to_owned(),
            spec_bytes: vec![],
            spec_type: "AgentSpec".into(),
        })),
    }
}

fn activity(run_id: &str, seq: u64, key: &str) -> JournalEvent {
    JournalEvent {
        event_id: format!("{}-act-{}", run_id, seq),
        run_id: run_id.to_owned(),
        seq,
        recorded_at_ns: 0,
        event: Some(Event::ActivityRecorded(ActivityRecordedEvent {
            activity_key: key.to_owned(),
            activity_kind: "test".to_owned(),
            input_json: "{}".to_owned(),
            result_json: r#""ok""#.to_owned(),
            replayed: false,
        })),
    }
}

fn completed(run_id: &str, seq: u64) -> JournalEvent {
    JournalEvent {
        event_id: format!("{}-done", run_id),
        run_id: run_id.to_owned(),
        seq,
        recorded_at_ns: 0,
        event: Some(Event::RunCompleted(RunCompletedEvent {
            output_json: r#"{"answer":"42"}"#.to_owned(),
        })),
    }
}

#[test]
fn replay_started_then_completed_yields_completed_status() {
    let run_id = "replay-1";
    let events = vec![started(run_id), completed(run_id, 1)];
    let state = replay_events(run_id, &events).unwrap();
    assert_eq!(state.run.status, RunStatus::Completed);
}

#[test]
fn replay_empty_journal_yields_pending() {
    let state = replay_events("run-empty", &[]).unwrap();
    assert_eq!(state.run.status, RunStatus::Pending);
}

#[test]
fn replay_preserves_activity_keys_in_order() {
    let run_id = "replay-acts";
    let events = vec![
        started(run_id),
        activity(run_id, 1, "step-1"),
        activity(run_id, 2, "step-2"),
        completed(run_id, 3),
    ];
    let state = replay_events(run_id, &events).unwrap();
    assert_eq!(state.activity_keys, vec!["step-1", "step-2"]);
}

#[test]
fn replay_is_deterministic_across_two_calls() {
    let run_id = "replay-det";
    let events = vec![
        started(run_id),
        activity(run_id, 1, "k"),
        completed(run_id, 2),
    ];
    let a = replay_events(run_id, &events).unwrap();
    let b = replay_events(run_id, &events).unwrap();
    assert_eq!(
        format!("{:?}", a.run.status),
        format!("{:?}", b.run.status),
        "replay must be deterministic"
    );
}

#[test]
fn memory_store_round_trip_through_append_and_read() {
    let store = MemoryStore::new();
    let run_id = "round-trip";
    let ev = started(run_id);
    store.append(run_id, ev.clone()).unwrap();
    let events = store.read(run_id).unwrap();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].run_id, run_id);
}

#[test]
fn replay_seq_advances_with_each_activity() {
    let run_id = "replay-seq";
    let n = 5u64;
    let mut events = vec![started(run_id)];
    for i in 1..=n {
        events.push(activity(run_id, i, &format!("step-{}", i)));
    }
    let state = replay_events(run_id, &events).unwrap();
    assert_eq!(state.run.seq, n, "run.seq must equal the last activity seq");
}

#[test]
fn replay_multiple_activity_keys_all_recovered() {
    let run_id = "replay-multi";
    let n = 10usize;
    let mut events = vec![started(run_id)];
    for i in 0..n {
        events.push(activity(run_id, (i + 1) as u64, &format!("key-{}", i)));
    }
    events.push(completed(run_id, (n + 1) as u64));

    let state = replay_events(run_id, &events).unwrap();
    assert_eq!(state.activity_keys.len(), n);
    for i in 0..n {
        let key = format!("key-{}", i);
        assert!(
            state.activity_keys.contains(&key),
            "key-{} must be present",
            i
        );
    }
}
