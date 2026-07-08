/// Distance metric parity -- cosine vs dot vs L2 across stores.
use ancora_core::{
    journal::{JournalStore, MemoryStore},
    replay::replay_events,
    run::RunStatus,
};
use ancora_proto::ancora::{
    journal_event::Event, ActivityRecordedEvent, JournalEvent, RunCompletedEvent, RunStartedEvent,
};

fn make_metric_journal(
    run_id: &str,
    metric: &str,
    store: &str,
    result_json: &str,
) -> Vec<JournalEvent> {
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
                activity_key: format!("retrieve-{}-{}", store, metric),
                activity_kind: "retrieval".into(),
                input_json: format!(
                    r#"{{"query":"metric test","top_k":2,"store":"{}","metric":"{}"}}"#,
                    store, metric
                ),
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
                output_json: r#"{"answer":"metric-ok"}"#.into(),
            })),
        },
    ]
}

const COSINE_RESULT: &str =
    r#"[{"text":"cosine hit A","score":0.95},{"text":"cosine hit B","score":0.88}]"#;
const DOT_RESULT: &str = r#"[{"text":"dot hit A","score":28.4},{"text":"dot hit B","score":24.1}]"#;
const L2_RESULT: &str =
    r#"[{"text":"l2 hit A","distance":0.06},{"text":"l2 hit B","distance":0.11}]"#;

#[test]
fn cosine_metric_pgvector_replays() {
    let events = make_metric_journal("pg-cos", "cosine", "pgvector", COSINE_RESULT);
    let store = MemoryStore::new();
    for ev in &events {
        store.append("pg-cos", ev.clone()).unwrap();
    }
    assert_eq!(
        replay_events("pg-cos", &store.read("pg-cos").unwrap())
            .unwrap()
            .run
            .status,
        RunStatus::Completed
    );
}

#[test]
fn dot_metric_milvus_replays() {
    let events = make_metric_journal("mv-dot", "IP", "milvus", DOT_RESULT);
    let store = MemoryStore::new();
    for ev in &events {
        store.append("mv-dot", ev.clone()).unwrap();
    }
    assert_eq!(
        replay_events("mv-dot", &store.read("mv-dot").unwrap())
            .unwrap()
            .run
            .status,
        RunStatus::Completed
    );
}

#[test]
fn l2_metric_lancedb_replays() {
    let events = make_metric_journal("ln-l2", "L2", "lancedb", L2_RESULT);
    let store = MemoryStore::new();
    for ev in &events {
        store.append("ln-l2", ev.clone()).unwrap();
    }
    assert_eq!(
        replay_events("ln-l2", &store.read("ln-l2").unwrap())
            .unwrap()
            .run
            .status,
        RunStatus::Completed
    );
}

#[test]
fn cosine_result_score_in_0_1_range() {
    let v: serde_json::Value = serde_json::from_str(COSINE_RESULT).unwrap();
    for chunk in v.as_array().unwrap() {
        let s = chunk["score"].as_f64().unwrap();
        assert!((0.0..=1.0).contains(&s), "cosine score out of range: {}", s);
    }
}

#[test]
fn l2_result_uses_distance_not_score() {
    let v: serde_json::Value = serde_json::from_str(L2_RESULT).unwrap();
    let first = &v.as_array().unwrap()[0];
    assert!(first.get("distance").is_some());
    assert!(first.get("score").is_none());
}

#[test]
fn metric_activity_key_encodes_both_store_and_metric() {
    for (rid, metric, store, result) in [
        ("pg-cos2", "cosine", "pgvector", COSINE_RESULT),
        ("mv-dot2", "IP", "milvus", DOT_RESULT),
    ] {
        let events = make_metric_journal(rid, metric, store, result);
        if let Some(Event::ActivityRecorded(a)) = &events[1].event {
            assert!(a.activity_key.contains(store));
            assert!(a.activity_key.contains(metric));
        }
    }
}
