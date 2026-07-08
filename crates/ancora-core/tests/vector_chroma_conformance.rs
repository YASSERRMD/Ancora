/// Chroma conformance -- offline fixture, no live Chroma.
use ancora_core::{
    journal::{JournalStore, MemoryStore},
    replay::replay_events,
    run::RunStatus,
};
use ancora_proto::ancora::{
    journal_event::Event, ActivityRecordedEvent, JournalEvent, RunCompletedEvent, RunStartedEvent,
};

fn build_chroma_journal(run_id: &str) -> Vec<JournalEvent> {
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
                activity_key: "retrieve-chroma".into(),
                activity_kind: "retrieval".into(),
                input_json: r#"{"query":"chroma docs","top_k":3,"collection_name":"my_collection","n_results":3}"#.into(),
                result_json: r#"[{"document":"Chroma doc A","distance":0.08,"id":"id-001","metadata":{"source":"pdf"}},{"document":"Chroma doc B","distance":0.13,"id":"id-002","metadata":{"source":"web"}},{"document":"Chroma doc C","distance":0.19,"id":"id-003","metadata":{"source":"txt"}}]"#.into(),
                replayed: false,
            })),
        },
        JournalEvent {
            event_id: format!("{}-2", run_id),
            run_id: run_id.to_string(),
            seq: 2,
            recorded_at_ns: 2_000,
            event: Some(Event::RunCompleted(RunCompletedEvent {
                output_json: r#"{"answer":"chroma-ok"}"#.into(),
            })),
        },
    ]
}

#[test]
fn chroma_journal_replays_to_completed() {
    let store = MemoryStore::new();
    let run_id = "chroma-run-151";
    for ev in &build_chroma_journal(run_id) {
        store.append(run_id, ev.clone()).unwrap();
    }
    let state = replay_events(run_id, &store.read(run_id).unwrap()).unwrap();
    assert_eq!(state.run.status, RunStatus::Completed);
}

#[test]
fn chroma_activity_key_contains_chroma() {
    let events = build_chroma_journal("r");
    if let Some(Event::ActivityRecorded(a)) = &events[1].event {
        assert!(a.activity_key.contains("chroma"));
    } else {
        panic!("Expected ActivityRecorded");
    }
}

#[test]
fn chroma_input_has_collection_name_and_n_results() {
    let events = build_chroma_journal("r");
    if let Some(Event::ActivityRecorded(a)) = &events[1].event {
        assert!(a.input_json.contains("collection_name"));
        assert!(a.input_json.contains("n_results"));
    } else {
        panic!("Expected ActivityRecorded");
    }
}

#[test]
fn chroma_result_uses_document_field() {
    let events = build_chroma_journal("r");
    if let Some(Event::ActivityRecorded(a)) = &events[1].event {
        assert!(a.result_json.contains("document"));
    } else {
        panic!("Expected ActivityRecorded");
    }
}

#[test]
fn chroma_result_has_metadata_with_source() {
    let events = build_chroma_journal("r");
    if let Some(Event::ActivityRecorded(a)) = &events[1].event {
        assert!(a.result_json.contains("metadata"));
        assert!(a.result_json.contains("source"));
    } else {
        panic!("Expected ActivityRecorded");
    }
}

#[test]
fn chroma_result_has_three_docs() {
    let events = build_chroma_journal("r");
    if let Some(Event::ActivityRecorded(a)) = &events[1].event {
        let v: serde_json::Value = serde_json::from_str(&a.result_json).unwrap();
        assert_eq!(v.as_array().unwrap().len(), 3);
    } else {
        panic!("Expected ActivityRecorded");
    }
}

#[test]
fn chroma_seq_monotonic() {
    let events = build_chroma_journal("r");
    for (i, ev) in events.iter().enumerate() {
        assert_eq!(ev.seq, i as u64);
    }
}
