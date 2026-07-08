/// Weaviate conformance -- offline fixture, no live Weaviate.
use ancora_core::{
    journal::{JournalStore, MemoryStore},
    replay::replay_events,
    run::RunStatus,
};
use ancora_proto::ancora::{
    journal_event::Event, ActivityRecordedEvent, JournalEvent, RunCompletedEvent, RunStartedEvent,
};

fn build_weaviate_journal(run_id: &str) -> Vec<JournalEvent> {
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
                activity_key: "retrieve-weaviate".into(),
                activity_kind: "retrieval".into(),
                input_json: r#"{"query":"weaviate test","top_k":2,"class_name":"KnowledgeBase"}"#.into(),
                result_json: r#"[{"text":"Weaviate chunk A","score":0.94,"_additional":{"certainty":0.94}},{"text":"Weaviate chunk B","score":0.88,"_additional":{"certainty":0.88}}]"#.into(),
                replayed: false,
            })),
        },
        JournalEvent {
            event_id: format!("{}-2", run_id),
            run_id: run_id.to_string(),
            seq: 2,
            recorded_at_ns: 2_000,
            event: Some(Event::RunCompleted(RunCompletedEvent {
                output_json: r#"{"answer":"weaviate-ok"}"#.into(),
            })),
        },
    ]
}

#[test]
fn weaviate_journal_replays_to_completed() {
    let store = MemoryStore::new();
    let run_id = "weaviate-run-151";
    for ev in &build_weaviate_journal(run_id) {
        store.append(run_id, ev.clone()).unwrap();
    }
    let state = replay_events(run_id, &store.read(run_id).unwrap()).unwrap();
    assert_eq!(state.run.status, RunStatus::Completed);
}

#[test]
fn weaviate_input_has_class_name() {
    let events = build_weaviate_journal("r");
    if let Some(Event::ActivityRecorded(a)) = &events[1].event {
        assert!(a.input_json.contains("class_name"));
    } else {
        panic!("Expected ActivityRecorded");
    }
}

#[test]
fn weaviate_result_has_additional_certainty() {
    let events = build_weaviate_journal("r");
    if let Some(Event::ActivityRecorded(a)) = &events[1].event {
        assert!(a.result_json.contains("_additional"));
        assert!(a.result_json.contains("certainty"));
    } else {
        panic!("Expected ActivityRecorded");
    }
}

#[test]
fn weaviate_result_has_two_chunks() {
    let events = build_weaviate_journal("r");
    if let Some(Event::ActivityRecorded(a)) = &events[1].event {
        let v: serde_json::Value = serde_json::from_str(&a.result_json).unwrap();
        assert_eq!(v.as_array().unwrap().len(), 2);
    } else {
        panic!("Expected ActivityRecorded");
    }
}

#[test]
fn weaviate_seq_monotonic() {
    let events = build_weaviate_journal("r");
    for (i, ev) in events.iter().enumerate() {
        assert_eq!(ev.seq, i as u64);
    }
}
