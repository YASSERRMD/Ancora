/// pgvector hybrid search (dense + BM25) -- offline fixture, no live Postgres.
use ancora_core::{
    journal::{JournalStore, MemoryStore},
    replay::replay_events,
    run::RunStatus,
};
use ancora_proto::ancora::{
    journal_event::Event, ActivityRecordedEvent, JournalEvent, RunCompletedEvent, RunStartedEvent,
};

fn build_pg_hybrid_journal(run_id: &str) -> Vec<JournalEvent> {
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
                activity_key: "retrieve-pgvector-hybrid".into(),
                activity_kind: "hybrid-retrieval".into(),
                input_json: r#"{"query":"pg hybrid","top_k":2,"alpha":0.7,"bm25_weight":0.3}"#.into(),
                result_json: r#"[{"text":"Hybrid PG A","dense_score":0.94,"bm25_score":0.72,"rrf_score":0.83},{"text":"Hybrid PG B","dense_score":0.88,"bm25_score":0.65,"rrf_score":0.77}]"#.into(),
                replayed: false,
            })),
        },
        JournalEvent {
            event_id: format!("{}-2", run_id),
            run_id: run_id.to_string(),
            seq: 2,
            recorded_at_ns: 2_000,
            event: Some(Event::RunCompleted(RunCompletedEvent {
                output_json: r#"{"answer":"pg-hybrid-ok"}"#.into(),
            })),
        },
    ]
}

#[test]
fn pgvector_hybrid_journal_replays_to_completed() {
    let store = MemoryStore::new();
    let run_id = "pg-hybrid-151";
    for ev in &build_pg_hybrid_journal(run_id) {
        store.append(run_id, ev.clone()).unwrap();
    }
    let state = replay_events(run_id, &store.read(run_id).unwrap()).unwrap();
    assert_eq!(state.run.status, RunStatus::Completed);
}

#[test]
fn pgvector_hybrid_activity_kind_is_hybrid_retrieval() {
    let events = build_pg_hybrid_journal("r");
    if let Some(Event::ActivityRecorded(a)) = &events[1].event {
        assert_eq!(a.activity_kind, "hybrid-retrieval");
    } else {
        panic!("Expected ActivityRecorded");
    }
}

#[test]
fn pgvector_hybrid_input_has_alpha() {
    let events = build_pg_hybrid_journal("r");
    if let Some(Event::ActivityRecorded(a)) = &events[1].event {
        assert!(a.input_json.contains("alpha"));
    } else {
        panic!("Expected ActivityRecorded");
    }
}

#[test]
fn pgvector_hybrid_result_has_rrf_score() {
    let events = build_pg_hybrid_journal("r");
    if let Some(Event::ActivityRecorded(a)) = &events[1].event {
        assert!(a.result_json.contains("rrf_score"));
    } else {
        panic!("Expected ActivityRecorded");
    }
}

#[test]
fn pgvector_hybrid_result_has_two_chunks() {
    let events = build_pg_hybrid_journal("r");
    if let Some(Event::ActivityRecorded(a)) = &events[1].event {
        let v: serde_json::Value = serde_json::from_str(&a.result_json).unwrap();
        assert_eq!(v.as_array().unwrap().len(), 2);
    } else {
        panic!("Expected ActivityRecorded");
    }
}

#[test]
fn pgvector_hybrid_seq_monotonic() {
    let events = build_pg_hybrid_journal("r");
    for (i, ev) in events.iter().enumerate() {
        assert_eq!(ev.seq, i as u64);
    }
}
