/// Qdrant hybrid sparse-dense -- offline fixture, no live Qdrant.
use ancora_core::{
    journal::{JournalStore, MemoryStore},
    replay::replay_events,
    run::RunStatus,
};
use ancora_proto::ancora::{
    journal_event::Event, ActivityRecordedEvent, JournalEvent, RunCompletedEvent, RunStartedEvent,
};

fn build_qdrant_hybrid_journal(run_id: &str) -> Vec<JournalEvent> {
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
                activity_key: "retrieve-qdrant-sparse-dense".into(),
                activity_kind: "hybrid-retrieval".into(),
                input_json: r#"{"query":"sparse dense","top_k":2,"sparse_vector_name":"sparse","dense_vector_name":"dense"}"#.into(),
                result_json: r#"[{"text":"QD hybrid A","dense_score":0.93,"sparse_score":0.81,"rrf":0.85},{"text":"QD hybrid B","dense_score":0.87,"sparse_score":0.74,"rrf":0.79}]"#.into(),
                replayed: false,
            })),
        },
        JournalEvent {
            event_id: format!("{}-2", run_id),
            run_id: run_id.to_string(),
            seq: 2,
            recorded_at_ns: 2_000,
            event: Some(Event::RunCompleted(RunCompletedEvent {
                output_json: r#"{"answer":"qdrant-sparse-dense-ok"}"#.into(),
            })),
        },
    ]
}

#[test]
fn qdrant_hybrid_journal_replays_to_completed() {
    let store = MemoryStore::new();
    let run_id = "qdrant-hybrid-151";
    for ev in &build_qdrant_hybrid_journal(run_id) {
        store.append(run_id, ev.clone()).unwrap();
    }
    let state = replay_events(run_id, &store.read(run_id).unwrap()).unwrap();
    assert_eq!(state.run.status, RunStatus::Completed);
}

#[test]
fn qdrant_hybrid_activity_kind_is_hybrid_retrieval() {
    let events = build_qdrant_hybrid_journal("r");
    if let Some(Event::ActivityRecorded(a)) = &events[1].event {
        assert_eq!(a.activity_kind, "hybrid-retrieval");
    } else { panic!("Expected ActivityRecorded"); }
}

#[test]
fn qdrant_hybrid_result_has_sparse_score() {
    let events = build_qdrant_hybrid_journal("r");
    if let Some(Event::ActivityRecorded(a)) = &events[1].event {
        assert!(a.result_json.contains("sparse_score"));
    } else { panic!("Expected ActivityRecorded"); }
}

#[test]
fn qdrant_hybrid_result_has_rrf() {
    let events = build_qdrant_hybrid_journal("r");
    if let Some(Event::ActivityRecorded(a)) = &events[1].event {
        assert!(a.result_json.contains("rrf"));
    } else { panic!("Expected ActivityRecorded"); }
}

#[test]
fn qdrant_hybrid_input_has_vector_names() {
    let events = build_qdrant_hybrid_journal("r");
    if let Some(Event::ActivityRecorded(a)) = &events[1].event {
        assert!(a.input_json.contains("sparse_vector_name"));
        assert!(a.input_json.contains("dense_vector_name"));
    } else { panic!("Expected ActivityRecorded"); }
}

#[test]
fn qdrant_hybrid_seq_monotonic() {
    let events = build_qdrant_hybrid_journal("r");
    for (i, ev) in events.iter().enumerate() {
        assert_eq!(ev.seq, i as u64);
    }
}
