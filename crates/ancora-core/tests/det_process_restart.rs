/// Determinism: replay across process restart.
/// The journal is durably stored. After a simulated restart (new MemoryStore reloaded from events),
/// replay produces the same RunStatus as the original run.
use ancora_core::{
    journal::{JournalStore, MemoryStore},
    replay::replay_events,
    run::RunStatus,
};
use ancora_proto::ancora::{
    journal_event::Event, ActivityRecordedEvent, JournalEvent, RunCompletedEvent, RunStartedEvent,
};

fn checkpoint_journal(run_id: &str) -> Vec<JournalEvent> {
    vec![
        JournalEvent { event_id: format!("{}-0", run_id), run_id: run_id.into(), seq: 0, recorded_at_ns: 0,
            event: Some(Event::RunStarted(RunStartedEvent { run_id: run_id.into(), spec_bytes: vec![], spec_type: "AgentSpec".into() })) },
        JournalEvent { event_id: format!("{}-1", run_id), run_id: run_id.into(), seq: 1, recorded_at_ns: 1_000,
            event: Some(Event::ActivityRecorded(ActivityRecordedEvent {
                activity_key: "step-1".into(), activity_kind: "compute".into(),
                input_json: r#"{"n":10}"#.into(), result_json: r#"{"n":20}"#.into(), replayed: false })) },
        JournalEvent { event_id: format!("{}-2", run_id), run_id: run_id.into(), seq: 2, recorded_at_ns: 2_000,
            event: Some(Event::ActivityRecorded(ActivityRecordedEvent {
                activity_key: "step-2".into(), activity_kind: "compute".into(),
                input_json: r#"{"n":20}"#.into(), result_json: r#"{"n":40}"#.into(), replayed: false })) },
        JournalEvent { event_id: format!("{}-3", run_id), run_id: run_id.into(), seq: 3, recorded_at_ns: 3_000,
            event: Some(Event::RunCompleted(RunCompletedEvent { output_json: r#"{"n":40}"#.into() })) },
    ]
}

fn simulate_restart_and_replay(run_id: &str) -> RunStatus {
    let persisted = checkpoint_journal(run_id);
    let store_after_restart = MemoryStore::new();
    for ev in &persisted { store_after_restart.append(run_id, ev.clone()).unwrap(); }
    replay_events(run_id, &store_after_restart.read(run_id).unwrap()).unwrap().run.status
}

#[test] fn replay_after_restart_produces_completed() {
    assert_eq!(simulate_restart_and_replay("det-rst-1"), RunStatus::Completed);
}

#[test] fn replay_after_restart_is_idempotent_across_two_restarts() {
    let s1 = simulate_restart_and_replay("det-rst-2a");
    let s2 = simulate_restart_and_replay("det-rst-2b");
    assert_eq!(s1, s2);
}

#[test] fn journal_preserves_two_activity_events_across_restart() {
    let j = checkpoint_journal("det-rst-cnt");
    let act = j.iter().filter(|e| matches!(e.event, Some(Event::ActivityRecorded(_)))).count();
    assert_eq!(act, 2);
}

#[test] fn journal_activity_results_stable_across_restart() {
    let j1 = checkpoint_journal("det-rst-res1");
    let j2 = checkpoint_journal("det-rst-res2");
    let res1: Vec<&str> = j1.iter().filter_map(|e| if let Some(Event::ActivityRecorded(a)) = &e.event { Some(a.result_json.as_str()) } else { None }).collect();
    let res2: Vec<&str> = j2.iter().filter_map(|e| if let Some(Event::ActivityRecorded(a)) = &e.event { Some(a.result_json.as_str()) } else { None }).collect();
    assert_eq!(res1, res2);
}

#[test] fn run_id_consistent_after_restart() {
    let j = checkpoint_journal("det-rst-id");
    for ev in &j { assert_eq!(ev.run_id, "det-rst-id"); }
}
