/// Weaviate hybrid (alpha-weighted fusion) -- offline fixture.
use ancora_core::{
    journal::{JournalStore, MemoryStore},
    replay::replay_events,
    run::RunStatus,
};
use ancora_proto::ancora::{
    journal_event::Event, ActivityRecordedEvent, JournalEvent, RunCompletedEvent, RunStartedEvent,
};

fn build_weaviate_hybrid_journal(run_id: &str) -> Vec<JournalEvent> {
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
                activity_key: "retrieve-weaviate-hybrid".into(),
                activity_kind: "hybrid-retrieval".into(),
                input_json: r#"{"query":"weaviate hybrid","top_k":3,"alpha":0.6,"class_name":"KnowledgeBase"}"#.into(),
                result_json: r#"[{"text":"WV hybrid A","vector_score":0.92,"bm25_score":0.76,"rrf_score":0.84},{"text":"WV hybrid B","vector_score":0.86,"bm25_score":0.70,"rrf_score":0.78},{"text":"WV hybrid C","vector_score":0.80,"bm25_score":0.62,"rrf_score":0.71}]"#.into(),
                replayed: false,
            })),
        },
        JournalEvent {
            event_id: format!("{}-2", run_id),
            run_id: run_id.to_string(),
            seq: 2,
            recorded_at_ns: 2_000,
            event: Some(Event::RunCompleted(RunCompletedEvent {
                output_json: r#"{"answer":"weaviate-hybrid-ok"}"#.into(),
            })),
        },
    ]
}

#[test]
fn weaviate_hybrid_replays_to_completed() {
    let store = MemoryStore::new();
    let run_id = "wv-hybrid-151";
    for ev in &build_weaviate_hybrid_journal(run_id) {
        store.append(run_id, ev.clone()).unwrap();
    }
    let state = replay_events(run_id, &store.read(run_id).unwrap()).unwrap();
    assert_eq!(state.run.status, RunStatus::Completed);
}

#[test]
fn weaviate_hybrid_activity_kind_is_hybrid_retrieval() {
    let events = build_weaviate_hybrid_journal("r");
    if let Some(Event::ActivityRecorded(a)) = &events[1].event {
        assert_eq!(a.activity_kind, "hybrid-retrieval");
    } else { panic!("Expected ActivityRecorded"); }
}

#[test]
fn weaviate_hybrid_input_has_alpha() {
    let events = build_weaviate_hybrid_journal("r");
    if let Some(Event::ActivityRecorded(a)) = &events[1].event {
        assert!(a.input_json.contains("alpha"));
    } else { panic!("Expected ActivityRecorded"); }
}

#[test]
fn weaviate_hybrid_result_has_rrf_score() {
    let events = build_weaviate_hybrid_journal("r");
    if let Some(Event::ActivityRecorded(a)) = &events[1].event {
        assert!(a.result_json.contains("rrf_score"));
    } else { panic!("Expected ActivityRecorded"); }
}

#[test]
fn weaviate_hybrid_result_has_three_chunks() {
    let events = build_weaviate_hybrid_journal("r");
    if let Some(Event::ActivityRecorded(a)) = &events[1].event {
        let v: serde_json::Value = serde_json::from_str(&a.result_json).unwrap();
        assert_eq!(v.as_array().unwrap().len(), 3);
    } else { panic!("Expected ActivityRecorded"); }
}

#[test]
fn weaviate_hybrid_seq_monotonic() {
    let events = build_weaviate_hybrid_journal("r");
    for (i, ev) in events.iter().enumerate() {
        assert_eq!(ev.seq, i as u64);
    }
}
