/// Determinism: partial journal resume.
/// A journal that is cut off before RunCompleted represents a run that can be resumed.
/// Replaying a partial journal should return a non-Completed status.
use ancora_core::{
    journal::{JournalStore, MemoryStore},
    replay::replay_events,
    run::RunStatus,
};
use ancora_proto::ancora::{
    journal_event::Event, ActivityRecordedEvent, JournalEvent, RunStartedEvent,
};

fn partial_journal(run_id: &str, steps_so_far: usize) -> Vec<JournalEvent> {
    let mut events = vec![
        JournalEvent { event_id: format!("{}-0", run_id), run_id: run_id.into(), seq: 0, recorded_at_ns: 0,
            event: Some(Event::RunStarted(RunStartedEvent { run_id: run_id.into(), spec_bytes: vec![], spec_type: "AgentSpec".into() })) },
    ];
    for i in 0..steps_so_far {
        events.push(JournalEvent { event_id: format!("{}-{}", run_id, i+1), run_id: run_id.into(), seq: (i+1) as u64, recorded_at_ns: ((i+1)*1000) as i64,
            event: Some(Event::ActivityRecorded(ActivityRecordedEvent {
                activity_key: format!("step-{}", i), activity_kind: "compute".into(),
                input_json: format!(r#"{{"n":{}}}"#, i), result_json: format!(r#"{{"n":{}}}"#, i+1), replayed: false })) });
    }
    events
}

#[test] fn partial_journal_only_started_is_not_completed() {
    let store = MemoryStore::new();
    let rid = "det-partial-0";
    for ev in &partial_journal(rid, 0) { store.append(rid, ev.clone()).unwrap(); }
    let state = replay_events(rid, &store.read(rid).unwrap()).unwrap();
    assert_ne!(state.run.status, RunStatus::Completed, "partial journal must not be Completed");
}

#[test] fn partial_journal_two_steps_is_not_completed() {
    let store = MemoryStore::new();
    let rid = "det-partial-2";
    for ev in &partial_journal(rid, 2) { store.append(rid, ev.clone()).unwrap(); }
    let state = replay_events(rid, &store.read(rid).unwrap()).unwrap();
    assert_ne!(state.run.status, RunStatus::Completed);
}

#[test] fn partial_journal_event_count_is_steps_plus_one() {
    assert_eq!(partial_journal("det-partial-cnt", 3).len(), 4);
}

#[test] fn partial_journal_run_id_consistent() {
    let rid = "det-partial-id";
    for ev in &partial_journal(rid, 3) { assert_eq!(ev.run_id, rid); }
}

#[test] fn partial_seq_monotonic() {
    let j = partial_journal("det-partial-seq", 5);
    for (i, ev) in j.iter().enumerate() { assert_eq!(ev.seq, i as u64); }
}
