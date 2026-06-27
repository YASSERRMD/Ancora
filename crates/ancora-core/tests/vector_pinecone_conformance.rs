/// Pinecone conformance -- offline mock fixture, no live Pinecone.
use ancora_core::{
    journal::{JournalStore, MemoryStore},
    replay::replay_events,
    run::RunStatus,
};
use ancora_proto::ancora::{
    journal_event::Event, ActivityRecordedEvent, JournalEvent, RunCompletedEvent, RunStartedEvent,
};

fn build_pinecone_journal(run_id: &str) -> Vec<JournalEvent> {
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
                activity_key: "retrieve-pinecone".into(),
                activity_kind: "retrieval".into(),
                input_json: r#"{"query":"pinecone search","top_k":2,"index_name":"prod-index","namespace":"default","environment":"us-east1-gcp"}"#.into(),
                result_json: r#"[{"id":"vec-001","score":0.96,"metadata":{"text":"Pinecone match A","source":"api"}},{"id":"vec-002","score":0.89,"metadata":{"text":"Pinecone match B","source":"web"}}]"#.into(),
                replayed: false,
            })),
        },
        JournalEvent {
            event_id: format!("{}-2", run_id),
            run_id: run_id.to_string(),
            seq: 2,
            recorded_at_ns: 2_000,
            event: Some(Event::RunCompleted(RunCompletedEvent {
                output_json: r#"{"answer":"pinecone-ok"}"#.into(),
            })),
        },
    ]
}

#[test]
fn pinecone_journal_replays_to_completed() {
    let store = MemoryStore::new();
    let run_id = "pinecone-run-151";
    for ev in &build_pinecone_journal(run_id) {
        store.append(run_id, ev.clone()).unwrap();
    }
    let state = replay_events(run_id, &store.read(run_id).unwrap()).unwrap();
    assert_eq!(state.run.status, RunStatus::Completed);
}

#[test]
fn pinecone_activity_key_contains_pinecone() {
    let events = build_pinecone_journal("r");
    if let Some(Event::ActivityRecorded(a)) = &events[1].event {
        assert!(a.activity_key.contains("pinecone"));
    } else { panic!("Expected ActivityRecorded"); }
}

#[test]
fn pinecone_input_has_index_namespace_environment() {
    let events = build_pinecone_journal("r");
    if let Some(Event::ActivityRecorded(a)) = &events[1].event {
        assert!(a.input_json.contains("index_name"));
        assert!(a.input_json.contains("namespace"));
        assert!(a.input_json.contains("environment"));
    } else { panic!("Expected ActivityRecorded"); }
}

#[test]
fn pinecone_result_uses_id_and_score_fields() {
    let events = build_pinecone_journal("r");
    if let Some(Event::ActivityRecorded(a)) = &events[1].event {
        let v: serde_json::Value = serde_json::from_str(&a.result_json).unwrap();
        let first = &v.as_array().unwrap()[0];
        assert!(first["id"].is_string());
        assert!(first["score"].is_number());
    } else { panic!("Expected ActivityRecorded"); }
}

#[test]
fn pinecone_result_metadata_has_text_field() {
    let events = build_pinecone_journal("r");
    if let Some(Event::ActivityRecorded(a)) = &events[1].event {
        assert!(a.result_json.contains("\"text\""));
    } else { panic!("Expected ActivityRecorded"); }
}

#[test]
fn pinecone_result_has_two_matches() {
    let events = build_pinecone_journal("r");
    if let Some(Event::ActivityRecorded(a)) = &events[1].event {
        let v: serde_json::Value = serde_json::from_str(&a.result_json).unwrap();
        assert_eq!(v.as_array().unwrap().len(), 2);
    } else { panic!("Expected ActivityRecorded"); }
}

#[test]
fn pinecone_seq_monotonic() {
    let events = build_pinecone_journal("r");
    for (i, ev) in events.iter().enumerate() {
        assert_eq!(ev.seq, i as u64);
    }
}
