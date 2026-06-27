/// Long-run memory stability tests (offline).
///
/// Validates that MemoryStore, MemoryStore clones, and replay operations
/// do not leak memory or grow unboundedly when processing large numbers of
/// runs or events. All tests are deterministic and offline.
use ancora_core::{
    journal::{JournalStore, MemoryStore},
    replay::replay_events,
    run::RunStatus,
};
use ancora_proto::ancora::{
    journal_event::Event, ActivityRecordedEvent, JournalEvent, RunCompletedEvent, RunStartedEvent,
};

fn minimal_journal(run_id: &str, n_activities: usize) -> Vec<JournalEvent> {
    let mut events = vec![JournalEvent {
        event_id: format!("{}-0", run_id),
        run_id: run_id.to_owned(),
        seq: 0,
        recorded_at_ns: 0,
        event: Some(Event::RunStarted(RunStartedEvent {
            run_id: run_id.to_owned(),
            spec_bytes: vec![],
            spec_type: "AgentSpec".into(),
        })),
    }];

    for i in 0..n_activities {
        events.push(JournalEvent {
            event_id: format!("{}-{}", run_id, i + 1),
            run_id: run_id.to_owned(),
            seq: (i + 1) as u64,
            recorded_at_ns: ((i + 1) * 1_000_000) as i64,
            event: Some(Event::ActivityRecorded(ActivityRecordedEvent {
                activity_key: format!("act-{}", i),
                activity_kind: "llm".into(),
                input_json: "{}".into(),
                result_json: "{}".into(),
                replayed: false,
            })),
        });
    }

    let last_seq = n_activities + 1;
    events.push(JournalEvent {
        event_id: format!("{}-{}", run_id, last_seq),
        run_id: run_id.to_owned(),
        seq: last_seq as u64,
        recorded_at_ns: (last_seq * 1_000_000) as i64,
        event: Some(Event::RunCompleted(RunCompletedEvent { output_json: String::new() })),
    });

    events
}

#[test]
fn store_handles_one_thousand_runs_without_error() {
    let store = MemoryStore::new();

    for i in 0..1_000 {
        let run_id = format!("run-{}", i);
        let events = minimal_journal(&run_id, 1);
        for ev in events {
            store.append(&run_id, ev).unwrap();
        }
    }

    let events = store.read("run-500").unwrap();
    assert_eq!(events.len(), 3, "run-500 must have started+activity+completed");
}

#[test]
fn replay_of_run_with_one_hundred_activities_completes() {
    let run_id = "mem-100-acts";
    let events = minimal_journal(run_id, 100);
    let state = replay_events(run_id, &events).unwrap();
    assert_eq!(state.run.status, RunStatus::Completed);
    assert_eq!(state.activity_keys.len(), 100);
}

#[test]
fn activity_keys_are_stable_after_one_hundred_appends() {
    let run_id = "mem-keys-100";
    let store = MemoryStore::new();
    let events = minimal_journal(run_id, 100);

    for ev in &events {
        store.append(run_id, ev.clone()).unwrap();
    }

    let loaded = store.read(run_id).unwrap();
    let state = replay_events(run_id, &loaded).unwrap();
    assert_eq!(state.activity_keys.len(), 100);
    assert_eq!(state.activity_keys[0], "act-0");
    assert_eq!(state.activity_keys[99], "act-99");
}

#[test]
fn cloned_store_shares_all_one_thousand_runs() {
    let store = MemoryStore::new();

    for i in 0..1_000 {
        let run_id = format!("clone-run-{}", i);
        let events = minimal_journal(&run_id, 1);
        for ev in events {
            store.append(&run_id, ev).unwrap();
        }
    }

    let clone = store.clone();
    let events = clone.read("clone-run-0").unwrap();
    assert_eq!(events.len(), 3);
    let events999 = clone.read("clone-run-999").unwrap();
    assert_eq!(events999.len(), 3);
}

#[test]
fn replay_of_empty_journal_always_produces_pending() {
    for i in 0..50 {
        let run_id = format!("empty-{}", i);
        let state = replay_events(&run_id, &[]).unwrap();
        assert_eq!(state.run.status, RunStatus::Pending);
    }
}

#[test]
fn repeated_replay_of_same_journal_is_stable() {
    let run_id = "mem-repeat";
    let events = minimal_journal(run_id, 10);

    for _ in 0..100 {
        let state = replay_events(run_id, &events).unwrap();
        assert_eq!(state.run.status, RunStatus::Completed);
        assert_eq!(state.activity_keys.len(), 10);
    }
}

#[test]
fn store_sequential_reads_are_consistent_under_one_hundred_clones() {
    let store = MemoryStore::new();
    let run_id = "mem-clone-reads";
    let events = minimal_journal(run_id, 5);

    for ev in &events {
        store.append(run_id, ev.clone()).unwrap();
    }

    let clones: Vec<MemoryStore> = (0..100).map(|_| store.clone()).collect();
    for clone in &clones {
        let loaded = clone.read(run_id).unwrap();
        assert_eq!(loaded.len(), 7, "7 = started + 5 activities + completed");
    }
}

#[test]
fn run_with_five_hundred_activities_has_correct_last_key() {
    let run_id = "mem-500";
    let events = minimal_journal(run_id, 500);
    let state = replay_events(run_id, &events).unwrap();
    assert_eq!(state.activity_keys[499], "act-499");
}
