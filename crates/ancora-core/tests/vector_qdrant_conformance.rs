/// Qdrant conformance -- offline fixture, no live Qdrant.
use ancora_core::{
    journal::{JournalStore, MemoryStore},
    replay::replay_events,
    run::RunStatus,
};
use ancora_proto::ancora::{
    journal_event::Event, ActivityRecordedEvent, JournalEvent, RunCompletedEvent, RunStartedEvent,
};

fn build_qdrant_journal(run_id: &str) -> Vec<JournalEvent> {
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
                activity_key: "retrieve-qdrant".into(),
                activity_kind: "retrieval".into(),
                input_json: r#"{"query":"qdrant docs","top_k":2,"collection":"kb"}"#.into(),
                result_json: r#"[{"text":"Qdrant chunk A","score":0.95},{"text":"Qdrant chunk B","score":0.89}]"#.into(),
                replayed: false,
            })),
        },
        JournalEvent {
            event_id: format!("{}-2", run_id),
            run_id: run_id.to_string(),
            seq: 2,
            recorded_at_ns: 2_000,
            event: Some(Event::RunCompleted(RunCompletedEvent {
                output_json: r#"{"answer":"qdrant-ok"}"#.into(),
            })),
        },
    ]
}

#[test]
fn qdrant_journal_replays_to_completed() {
    let store = MemoryStore::new();
    let run_id = "qdrant-run-151";
    for ev in &build_qdrant_journal(run_id) {
        store.append(run_id, ev.clone()).unwrap();
    }
    let state = replay_events(run_id, &store.read(run_id).unwrap()).unwrap();
    assert_eq!(state.run.status, RunStatus::Completed);
}

#[test]
fn qdrant_activity_key_contains_qdrant() {
    let events = build_qdrant_journal("r");
    if let Some(Event::ActivityRecorded(a)) = &events[1].event {
        assert!(a.activity_key.contains("qdrant"));
    } else { panic!("Expected ActivityRecorded"); }
}

#[test]
fn qdrant_input_json_has_collection() {
    let events = build_qdrant_journal("r");
    if let Some(Event::ActivityRecorded(a)) = &events[1].event {
        assert!(a.input_json.contains("collection"));
    } else { panic!("Expected ActivityRecorded"); }
}

#[test]
fn qdrant_result_has_two_chunks() {
    let events = build_qdrant_journal("r");
    if let Some(Event::ActivityRecorded(a)) = &events[1].event {
        let v: serde_json::Value = serde_json::from_str(&a.result_json).unwrap();
        assert_eq!(v.as_array().unwrap().len(), 2);
    } else { panic!("Expected ActivityRecorded"); }
}

#[test]
fn qdrant_seq_monotonic() {
    let events = build_qdrant_journal("r");
    for (i, ev) in events.iter().enumerate() {
        assert_eq!(ev.seq, i as u64);
    }
}
