/// Milvus partition-aware search -- offline fixture.
use ancora_core::{
    journal::{JournalStore, MemoryStore},
    replay::replay_events,
    run::RunStatus,
};
use ancora_proto::ancora::{
    journal_event::Event, ActivityRecordedEvent, JournalEvent, RunCompletedEvent, RunStartedEvent,
};

fn build_milvus_partitions_journal(run_id: &str) -> Vec<JournalEvent> {
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
                activity_key: "retrieve-milvus-partitions".into(),
                activity_kind: "retrieval".into(),
                input_json: r#"{"query":"partition query","top_k":2,"collection_name":"docs","partitions":["en","zh"],"metric_type":"L2"}"#.into(),
                result_json: r#"[{"text":"Partition EN chunk","score":0.93,"partition":"en","id":2001},{"text":"Partition ZH chunk","score":0.87,"partition":"zh","id":2002}]"#.into(),
                replayed: false,
            })),
        },
        JournalEvent {
            event_id: format!("{}-2", run_id),
            run_id: run_id.to_string(),
            seq: 2,
            recorded_at_ns: 2_000,
            event: Some(Event::RunCompleted(RunCompletedEvent {
                output_json: r#"{"answer":"milvus-partitions-ok"}"#.into(),
            })),
        },
    ]
}

#[test]
fn milvus_partitions_replays_to_completed() {
    let store = MemoryStore::new();
    let run_id = "milvus-part-151";
    for ev in &build_milvus_partitions_journal(run_id) {
        store.append(run_id, ev.clone()).unwrap();
    }
    let state = replay_events(run_id, &store.read(run_id).unwrap()).unwrap();
    assert_eq!(state.run.status, RunStatus::Completed);
}

#[test]
fn milvus_partitions_input_has_partitions_array() {
    let events = build_milvus_partitions_journal("r");
    if let Some(Event::ActivityRecorded(a)) = &events[1].event {
        let v: serde_json::Value = serde_json::from_str(&a.input_json).unwrap();
        let parts = v["partitions"].as_array().unwrap();
        assert_eq!(parts.len(), 2);
        assert_eq!(parts[0], "en");
        assert_eq!(parts[1], "zh");
    } else { panic!("Expected ActivityRecorded"); }
}

#[test]
fn milvus_partitions_result_includes_partition_field() {
    let events = build_milvus_partitions_journal("r");
    if let Some(Event::ActivityRecorded(a)) = &events[1].event {
        assert!(a.result_json.contains("partition"));
    } else { panic!("Expected ActivityRecorded"); }
}

#[test]
fn milvus_partitions_result_has_two_chunks() {
    let events = build_milvus_partitions_journal("r");
    if let Some(Event::ActivityRecorded(a)) = &events[1].event {
        let v: serde_json::Value = serde_json::from_str(&a.result_json).unwrap();
        assert_eq!(v.as_array().unwrap().len(), 2);
    } else { panic!("Expected ActivityRecorded"); }
}

#[test]
fn milvus_partitions_metric_is_l2() {
    let events = build_milvus_partitions_journal("r");
    if let Some(Event::ActivityRecorded(a)) = &events[1].event {
        assert!(a.input_json.contains("L2"));
    } else { panic!("Expected ActivityRecorded"); }
}

#[test]
fn milvus_partitions_seq_monotonic() {
    let events = build_milvus_partitions_journal("r");
    for (i, ev) in events.iter().enumerate() {
        assert_eq!(ev.seq, i as u64);
    }
}
