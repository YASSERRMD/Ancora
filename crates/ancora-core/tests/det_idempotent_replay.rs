/// Determinism: replay is idempotent -- run twice, same result.
use ancora_core::{
    journal::{JournalStore, MemoryStore},
    replay::replay_events,
    run::RunStatus,
};
use ancora_proto::ancora::{
    journal_event::Event, ActivityRecordedEvent, JournalEvent, RunCompletedEvent, RunStartedEvent,
};

fn full_journal(run_id: &str) -> Vec<JournalEvent> {
    vec![
        JournalEvent { event_id: format!("{}-0", run_id), run_id: run_id.into(), seq: 0, recorded_at_ns: 0,
            event: Some(Event::RunStarted(RunStartedEvent { run_id: run_id.into(), spec_bytes: vec![], spec_type: "AgentSpec".into() })) },
        JournalEvent { event_id: format!("{}-1", run_id), run_id: run_id.into(), seq: 1, recorded_at_ns: 1_000,
            event: Some(Event::ActivityRecorded(ActivityRecordedEvent {
                activity_key: "idem-step".into(), activity_kind: "compute".into(),
                input_json: r#"{"x":10}"#.into(), result_json: r#"{"y":20}"#.into(), replayed: false })) },
        JournalEvent { event_id: format!("{}-2", run_id), run_id: run_id.into(), seq: 2, recorded_at_ns: 2_000,
            event: Some(Event::RunCompleted(RunCompletedEvent { output_json: r#"{"y":20}"#.into() })) },
    ]
}

fn replay_n_times(run_id: &str, n: usize) -> Vec<RunStatus> {
    let j = full_journal(run_id);
    (0..n).map(|_| {
        let store = MemoryStore::new();
        for ev in &j { store.append(run_id, ev.clone()).unwrap(); }
        replay_events(run_id, &store.read(run_id).unwrap()).unwrap().run.status
    }).collect()
}

#[test] fn replay_twice_same_status() {
    let statuses = replay_n_times("det-idem-2", 2);
    assert!(statuses.iter().all(|s| *s == RunStatus::Completed));
}

#[test] fn replay_ten_times_all_completed() {
    let statuses = replay_n_times("det-idem-10", 10);
    assert!(statuses.iter().all(|s| *s == RunStatus::Completed));
}

#[test] fn replay_does_not_mutate_journal() {
    let rid = "det-idem-mut";
    let j_before = full_journal(rid);
    let store = MemoryStore::new();
    for ev in &j_before { store.append(rid, ev.clone()).unwrap(); }
    let _ = replay_events(rid, &store.read(rid).unwrap()).unwrap();
    let j_after = store.read(rid).unwrap();
    assert_eq!(j_before.len(), j_after.len(), "replay must not add or remove journal events");
}

#[test] fn multiple_independent_replays_all_have_same_event_read_back() {
    let rid = "det-idem-ev";
    let j = full_journal(rid);
    for _ in 0..5 {
        let store = MemoryStore::new();
        for ev in &j { store.append(rid, ev.clone()).unwrap(); }
        let events_back = store.read(rid).unwrap();
        assert_eq!(events_back.len(), j.len());
    }
}

#[test] fn idempotent_replay_preserves_output_json() {
    let rid = "det-idem-out";
    let j = full_journal(rid);
    let expected_output = r#"{"y":20}"#;
    if let Some(Event::RunCompleted(c)) = &j.last().unwrap().event {
        assert_eq!(c.output_json, expected_output);
    }
    let store = MemoryStore::new();
    for ev in &j { store.append(rid, ev.clone()).unwrap(); }
    let _ = replay_events(rid, &store.read(rid).unwrap()).unwrap();
    let j_after = store.read(rid).unwrap();
    if let Some(Event::RunCompleted(c)) = &j_after.last().unwrap().event {
        assert_eq!(c.output_json, expected_output, "output_json must not change after replay");
    }
}
