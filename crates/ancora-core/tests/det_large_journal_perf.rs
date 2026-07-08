/// Determinism: large journal replay performance.
/// A journal with 500 activity events should replay within a reasonable time budget.
use ancora_core::{
    journal::{JournalStore, MemoryStore},
    replay::replay_events,
    run::RunStatus,
};
use ancora_proto::ancora::{
    journal_event::Event, ActivityRecordedEvent, JournalEvent, RunCompletedEvent, RunStartedEvent,
};
use std::time::Instant;

const LARGE_N: usize = 500;

fn large_journal(run_id: &str) -> Vec<JournalEvent> {
    let mut events = vec![JournalEvent {
        event_id: format!("{}-0", run_id),
        run_id: run_id.into(),
        seq: 0,
        recorded_at_ns: 0,
        event: Some(Event::RunStarted(RunStartedEvent {
            run_id: run_id.into(),
            spec_bytes: vec![],
            spec_type: "AgentSpec".into(),
        })),
    }];
    for i in 0..LARGE_N {
        events.push(JournalEvent {
            event_id: format!("{}-{}", run_id, i + 1),
            run_id: run_id.into(),
            seq: (i + 1) as u64,
            recorded_at_ns: ((i + 1) * 1000) as i64,
            event: Some(Event::ActivityRecorded(ActivityRecordedEvent {
                activity_key: format!("step-{}", i),
                activity_kind: "compute".into(),
                input_json: format!(r#"{{"n":{}}}"#, i),
                result_json: format!(r#"{{"n":{}}}"#, i * 2),
                replayed: false,
            })),
        });
    }
    let n = events.len();
    events.push(JournalEvent {
        event_id: format!("{}-{}", run_id, n),
        run_id: run_id.into(),
        seq: n as u64,
        recorded_at_ns: (n * 1000) as i64,
        event: Some(Event::RunCompleted(RunCompletedEvent {
            output_json: r#"{"done":true}"#.into(),
        })),
    });
    events
}

#[test]
fn large_journal_replays_to_completed() {
    let store = MemoryStore::new();
    let rid = "det-large-perf";
    for ev in &large_journal(rid) {
        store.append(rid, ev.clone()).unwrap();
    }
    let status = replay_events(rid, &store.read(rid).unwrap())
        .unwrap()
        .run
        .status;
    assert_eq!(status, RunStatus::Completed);
}

#[test]
fn large_journal_has_correct_event_count() {
    let j = large_journal("det-large-cnt");
    assert_eq!(j.len(), LARGE_N + 2);
}

#[test]
fn large_journal_replay_completes_within_time_budget() {
    let store = MemoryStore::new();
    let rid = "det-large-time";
    for ev in &large_journal(rid) {
        store.append(rid, ev.clone()).unwrap();
    }
    let start = Instant::now();
    let _ = replay_events(rid, &store.read(rid).unwrap()).unwrap();
    let elapsed = start.elapsed();
    assert!(elapsed.as_secs() < 5, "replay took too long: {:?}", elapsed);
}

#[test]
fn large_journal_seq_monotonic() {
    let j = large_journal("det-large-seq");
    for (i, ev) in j.iter().enumerate() {
        assert_eq!(ev.seq, i as u64);
    }
}

#[test]
fn large_journal_all_activity_keys_unique() {
    let j = large_journal("det-large-keys");
    let keys: std::collections::HashSet<String> = j
        .iter()
        .filter_map(|e| {
            if let Some(Event::ActivityRecorded(a)) = &e.event {
                Some(a.activity_key.clone())
            } else {
                None
            }
        })
        .collect();
    assert_eq!(keys.len(), LARGE_N, "all activity keys must be unique");
}
