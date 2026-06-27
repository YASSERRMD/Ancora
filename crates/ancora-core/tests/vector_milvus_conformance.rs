/// Milvus conformance -- offline fixture, no live Milvus.
use ancora_core::{
    journal::{JournalStore, MemoryStore},
    replay::replay_events,
    run::RunStatus,
};
use ancora_proto::ancora::{
    journal_event::Event, ActivityRecordedEvent, JournalEvent, RunCompletedEvent, RunStartedEvent,
};

fn build_milvus_journal(run_id: &str) -> Vec<JournalEvent> {
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
                activity_key: "retrieve-milvus".into(),
                activity_kind: "retrieval".into(),
                input_json: r#"{"query":"milvus docs","top_k":3,"collection_name":"knowledge","metric_type":"IP"}"#.into(),
                result_json: r#"[{"text":"Milvus chunk A","score":0.97,"id":1001},{"text":"Milvus chunk B","score":0.91,"id":1002},{"text":"Milvus chunk C","score":0.85,"id":1003}]"#.into(),
                replayed: false,
            })),
        },
        JournalEvent {
            event_id: format!("{}-2", run_id),
            run_id: run_id.to_string(),
            seq: 2,
            recorded_at_ns: 2_000,
            event: Some(Event::RunCompleted(RunCompletedEvent {
                output_json: r#"{"answer":"milvus-ok"}"#.into(),
            })),
        },
    ]
}

#[test]
fn milvus_journal_replays_to_completed() {
    let store = MemoryStore::new();
    let run_id = "milvus-run-151";
    for ev in &build_milvus_journal(run_id) {
        store.append(run_id, ev.clone()).unwrap();
    }
    let state = replay_events(run_id, &store.read(run_id).unwrap()).unwrap();
    assert_eq!(state.run.status, RunStatus::Completed);
}

#[test]
fn milvus_activity_key_contains_milvus() {
    let events = build_milvus_journal("r");
    if let Some(Event::ActivityRecorded(a)) = &events[1].event {
        assert!(a.activity_key.contains("milvus"));
    } else { panic!("Expected ActivityRecorded"); }
}

#[test]
fn milvus_input_has_collection_name_and_metric() {
    let events = build_milvus_journal("r");
    if let Some(Event::ActivityRecorded(a)) = &events[1].event {
        assert!(a.input_json.contains("collection_name"));
        assert!(a.input_json.contains("metric_type"));
    } else { panic!("Expected ActivityRecorded"); }
}

#[test]
fn milvus_result_has_three_chunks_with_ids() {
    let events = build_milvus_journal("r");
    if let Some(Event::ActivityRecorded(a)) = &events[1].event {
        let v: serde_json::Value = serde_json::from_str(&a.result_json).unwrap();
        let arr = v.as_array().unwrap();
        assert_eq!(arr.len(), 3);
        assert!(arr[0]["id"].is_number());
    } else { panic!("Expected ActivityRecorded"); }
}

#[test]
fn milvus_scores_descending() {
    let events = build_milvus_journal("r");
    if let Some(Event::ActivityRecorded(a)) = &events[1].event {
        let v: serde_json::Value = serde_json::from_str(&a.result_json).unwrap();
        let arr = v.as_array().unwrap();
        let s0 = arr[0]["score"].as_f64().unwrap();
        let s1 = arr[1]["score"].as_f64().unwrap();
        let s2 = arr[2]["score"].as_f64().unwrap();
        assert!(s0 >= s1 && s1 >= s2);
    } else { panic!("Expected ActivityRecorded"); }
}

#[test]
fn milvus_seq_monotonic() {
    let events = build_milvus_journal("r");
    for (i, ev) in events.iter().enumerate() {
        assert_eq!(ev.seq, i as u64);
    }
}
