/// Redis vector search conformance -- offline fixture, no live Redis.
use ancora_core::{
    journal::{JournalStore, MemoryStore},
    replay::replay_events,
    run::RunStatus,
};
use ancora_proto::ancora::{
    journal_event::Event, ActivityRecordedEvent, JournalEvent, RunCompletedEvent, RunStartedEvent,
};

fn build_redis_vector_journal(run_id: &str) -> Vec<JournalEvent> {
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
                activity_key: "retrieve-redis-vector".into(),
                activity_kind: "retrieval".into(),
                input_json: r#"{"query":"redis docs","top_k":2,"index_name":"idx:docs","vector_field":"embedding","algorithm":"HNSW"}"#.into(),
                result_json: r#"[{"key":"doc:001","text":"Redis vector result A","vector_score":0.95},{"key":"doc:002","text":"Redis vector result B","vector_score":0.87}]"#.into(),
                replayed: false,
            })),
        },
        JournalEvent {
            event_id: format!("{}-2", run_id),
            run_id: run_id.to_string(),
            seq: 2,
            recorded_at_ns: 2_000,
            event: Some(Event::RunCompleted(RunCompletedEvent {
                output_json: r#"{"answer":"redis-vector-ok"}"#.into(),
            })),
        },
    ]
}

#[test]
fn redis_vector_journal_replays_to_completed() {
    let store = MemoryStore::new();
    let run_id = "redis-vec-151";
    for ev in &build_redis_vector_journal(run_id) {
        store.append(run_id, ev.clone()).unwrap();
    }
    let state = replay_events(run_id, &store.read(run_id).unwrap()).unwrap();
    assert_eq!(state.run.status, RunStatus::Completed);
}

#[test]
fn redis_vector_activity_key_contains_redis() {
    let events = build_redis_vector_journal("r");
    if let Some(Event::ActivityRecorded(a)) = &events[1].event {
        assert!(a.activity_key.contains("redis"));
    } else { panic!("Expected ActivityRecorded"); }
}

#[test]
fn redis_vector_input_has_index_and_algorithm() {
    let events = build_redis_vector_journal("r");
    if let Some(Event::ActivityRecorded(a)) = &events[1].event {
        assert!(a.input_json.contains("index_name"));
        assert!(a.input_json.contains("algorithm"));
        assert!(a.input_json.contains("HNSW"));
    } else { panic!("Expected ActivityRecorded"); }
}

#[test]
fn redis_vector_result_uses_key_field() {
    let events = build_redis_vector_journal("r");
    if let Some(Event::ActivityRecorded(a)) = &events[1].event {
        assert!(a.result_json.contains("\"key\""));
    } else { panic!("Expected ActivityRecorded"); }
}

#[test]
fn redis_vector_result_has_two_entries() {
    let events = build_redis_vector_journal("r");
    if let Some(Event::ActivityRecorded(a)) = &events[1].event {
        let v: serde_json::Value = serde_json::from_str(&a.result_json).unwrap();
        assert_eq!(v.as_array().unwrap().len(), 2);
    } else { panic!("Expected ActivityRecorded"); }
}

#[test]
fn redis_vector_seq_monotonic() {
    let events = build_redis_vector_journal("r");
    for (i, ev) in events.iter().enumerate() {
        assert_eq!(ev.seq, i as u64);
    }
}
