/// SQLite vector store conformance -- offline, no live store.
use ancora_core::{
    journal::{JournalStore, MemoryStore},
    replay::replay_events,
    run::RunStatus,
};
use ancora_proto::ancora::{
    journal_event::Event, ActivityRecordedEvent, JournalEvent, RunCompletedEvent, RunStartedEvent,
};

fn build_sqlite_journal(run_id: &str, chunks: &[(&str, f32)]) -> Vec<JournalEvent> {
    let result: String = {
        let items: Vec<String> = chunks
            .iter()
            .map(|(t, s)| {
                format!(
                    r#"{{"text":{},"score":{}}}"#,
                    serde_json::to_string(t).unwrap(),
                    s
                )
            })
            .collect();
        format!("[{}]", items.join(","))
    };
    vec![
        JournalEvent {
            event_id: format!("{}-0", run_id),
            run_id: run_id.to_string(),
            seq: 0,
            recorded_at_ns: 0,
            event: Some(Event::RunStarted(RunStartedEvent {
                run_id: run_id.to_string(),
                spec_bytes: vec![],
                spec_type: "AgentSpec".into(),
            })),
        },
        JournalEvent {
            event_id: format!("{}-1", run_id),
            run_id: run_id.to_string(),
            seq: 1,
            recorded_at_ns: 1_000,
            event: Some(Event::ActivityRecorded(ActivityRecordedEvent {
                activity_key: "retrieve-sqlite-vec".into(),
                activity_kind: "retrieval".into(),
                input_json: r#"{"query":"local db","top_k":2}"#.into(),
                result_json: result,
                replayed: false,
            })),
        },
        JournalEvent {
            event_id: format!("{}-2", run_id),
            run_id: run_id.to_string(),
            seq: 2,
            recorded_at_ns: 2_000,
            event: Some(Event::RunCompleted(RunCompletedEvent {
                output_json: r#"{"answer":"sqlite-ok"}"#.into(),
            })),
        },
    ]
}

const CHUNKS: &[(&str, f32)] = &[("SQLite-vec chunk A", 0.92), ("SQLite-vec chunk B", 0.86)];

#[test]
fn sqlite_journal_replays_to_completed() {
    let store = MemoryStore::new();
    let run_id = "sqlite-run-151";
    for ev in &build_sqlite_journal(run_id, CHUNKS) {
        store.append(run_id, ev.clone()).unwrap();
    }
    let state = replay_events(run_id, &store.read(run_id).unwrap()).unwrap();
    assert_eq!(state.run.status, RunStatus::Completed);
}

#[test]
fn sqlite_activity_key_has_sqlite_label() {
    let events = build_sqlite_journal("r", CHUNKS);
    if let Some(Event::ActivityRecorded(a)) = &events[1].event {
        assert!(a.activity_key.contains("sqlite"));
    } else {
        panic!("Expected ActivityRecorded");
    }
}

#[test]
fn sqlite_result_json_is_two_chunks() {
    let events = build_sqlite_journal("r", CHUNKS);
    if let Some(Event::ActivityRecorded(a)) = &events[1].event {
        let v: serde_json::Value = serde_json::from_str(&a.result_json).unwrap();
        assert_eq!(v.as_array().unwrap().len(), 2);
    } else {
        panic!("Expected ActivityRecorded");
    }
}

#[test]
fn sqlite_seq_monotonic() {
    let events = build_sqlite_journal("r", CHUNKS);
    for (i, ev) in events.iter().enumerate() {
        assert_eq!(ev.seq, i as u64);
    }
}

#[test]
fn sqlite_run_ids_match() {
    let run_id = "sqlite-match";
    for ev in &build_sqlite_journal(run_id, CHUNKS) {
        assert_eq!(ev.run_id, run_id);
    }
}
