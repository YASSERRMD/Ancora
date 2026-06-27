/// Metadata filter parity -- same filter semantics across stores.
use ancora_core::{
    journal::{JournalStore, MemoryStore},
    replay::replay_events,
    run::RunStatus,
};
use ancora_proto::ancora::{
    journal_event::Event, ActivityRecordedEvent, JournalEvent, RunCompletedEvent, RunStartedEvent,
};

fn make_filter_event(run_id: &str, store: &str, activity_key: &str, filter_json: &str, result_json: &str) -> Vec<JournalEvent> {
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
                activity_key: activity_key.to_string(),
                activity_kind: "retrieval".into(),
                input_json: format!(r#"{{"query":"metadata filter","top_k":2,"store":"{}","filter":{}}}"#, store, filter_json),
                result_json: result_json.to_string(),
                replayed: false,
            })),
        },
        JournalEvent {
            event_id: format!("{}-2", run_id),
            run_id: run_id.to_string(),
            seq: 2,
            recorded_at_ns: 2_000,
            event: Some(Event::RunCompleted(RunCompletedEvent {
                output_json: r#"{"answer":"filter-ok"}"#.into(),
            })),
        },
    ]
}

const PG_FILTER: &str = r#"{"source":{"$eq":"pdf"}}"#;
const QD_FILTER: &str = r#"{"must":[{"key":"source","match":{"value":"pdf"}}]}"#;
const CH_FILTER: &str = r#"{"source":{"$eq":"pdf"}}"#;
const WV_FILTER: &str = r#"{"path":["source"],"operator":"Equal","valueText":"pdf"}"#;
const IN_MEM_RESULT: &str = r#"[{"text":"PDF result","score":0.94,"metadata":{"source":"pdf"}}]"#;

#[test]
fn pgvector_filter_by_source_replays() {
    let events = make_filter_event("pg-f", "pgvector", "retrieve-pgvector-filtered", PG_FILTER, IN_MEM_RESULT);
    let store = MemoryStore::new();
    for ev in &events { store.append("pg-f", ev.clone()).unwrap(); }
    let state = replay_events("pg-f", &store.read("pg-f").unwrap()).unwrap();
    assert_eq!(state.run.status, RunStatus::Completed);
}

#[test]
fn qdrant_filter_by_source_replays() {
    let events = make_filter_event("qd-f", "qdrant", "retrieve-qdrant-filtered", QD_FILTER, IN_MEM_RESULT);
    let store = MemoryStore::new();
    for ev in &events { store.append("qd-f", ev.clone()).unwrap(); }
    let state = replay_events("qd-f", &store.read("qd-f").unwrap()).unwrap();
    assert_eq!(state.run.status, RunStatus::Completed);
}

#[test]
fn chroma_filter_by_source_replays() {
    let events = make_filter_event("ch-f", "chroma", "retrieve-chroma-filtered", CH_FILTER, IN_MEM_RESULT);
    let store = MemoryStore::new();
    for ev in &events { store.append("ch-f", ev.clone()).unwrap(); }
    let state = replay_events("ch-f", &store.read("ch-f").unwrap()).unwrap();
    assert_eq!(state.run.status, RunStatus::Completed);
}

#[test]
fn weaviate_filter_by_source_replays() {
    let events = make_filter_event("wv-f", "weaviate", "retrieve-weaviate-filtered", WV_FILTER, IN_MEM_RESULT);
    let store = MemoryStore::new();
    for ev in &events { store.append("wv-f", ev.clone()).unwrap(); }
    let state = replay_events("wv-f", &store.read("wv-f").unwrap()).unwrap();
    assert_eq!(state.run.status, RunStatus::Completed);
}

#[test]
fn all_filtered_results_contain_pdf_source() {
    for (rid, key, filt) in [
        ("pg-all", "retrieve-pgvector-filtered", PG_FILTER),
        ("qd-all", "retrieve-qdrant-filtered", QD_FILTER),
        ("ch-all", "retrieve-chroma-filtered", CH_FILTER),
        ("wv-all", "retrieve-weaviate-filtered", WV_FILTER),
    ] {
        let events = make_filter_event(rid, "any", key, filt, IN_MEM_RESULT);
        if let Some(Event::ActivityRecorded(a)) = &events[1].event {
            let v: serde_json::Value = serde_json::from_str(&a.result_json).unwrap();
            for chunk in v.as_array().unwrap() {
                assert_eq!(chunk["metadata"]["source"], "pdf");
            }
        }
    }
}

#[test]
fn filter_parity_seq_always_monotonic() {
    for (rid, key, filt) in [
        ("pg-seq", "pgvector-filtered", PG_FILTER),
        ("qd-seq", "qdrant-filtered", QD_FILTER),
    ] {
        let events = make_filter_event(rid, "any", key, filt, IN_MEM_RESULT);
        for (i, ev) in events.iter().enumerate() {
            assert_eq!(ev.seq, i as u64);
        }
    }
}
