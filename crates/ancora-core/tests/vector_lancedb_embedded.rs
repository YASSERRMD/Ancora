/// LanceDB embedded offline mode -- purely local, no server.
use ancora_core::{
    journal::{JournalStore, MemoryStore},
    replay::replay_events,
    run::RunStatus,
};
use ancora_proto::ancora::{
    journal_event::Event, ActivityRecordedEvent, JournalEvent, RunCompletedEvent, RunStartedEvent,
};

fn build_lancedb_embedded_journal(run_id: &str) -> Vec<JournalEvent> {
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
                activity_key: "retrieve-lancedb-embedded".into(),
                activity_kind: "retrieval".into(),
                input_json: r#"{"query":"embedded offline","top_k":2,"table":"local_kb","uri":"/tmp/lance_embedded_test","mode":"embedded"}"#.into(),
                result_json: r#"[{"text":"Embedded chunk 1","score":0.96,"source":"local"},{"text":"Embedded chunk 2","score":0.89,"source":"local"}]"#.into(),
                replayed: false,
            })),
        },
        JournalEvent {
            event_id: format!("{}-2", run_id),
            run_id: run_id.to_string(),
            seq: 2,
            recorded_at_ns: 2_000,
            event: Some(Event::RunCompleted(RunCompletedEvent {
                output_json: r#"{"answer":"lancedb-embedded-ok"}"#.into(),
            })),
        },
    ]
}

#[test]
fn lancedb_embedded_replays_to_completed() {
    let store = MemoryStore::new();
    let run_id = "lance-emb-151";
    for ev in &build_lancedb_embedded_journal(run_id) {
        store.append(run_id, ev.clone()).unwrap();
    }
    let state = replay_events(run_id, &store.read(run_id).unwrap()).unwrap();
    assert_eq!(state.run.status, RunStatus::Completed);
}

#[test]
fn lancedb_embedded_mode_field_is_embedded() {
    let events = build_lancedb_embedded_journal("r");
    if let Some(Event::ActivityRecorded(a)) = &events[1].event {
        let v: serde_json::Value = serde_json::from_str(&a.input_json).unwrap();
        assert_eq!(v["mode"], "embedded");
    } else {
        panic!("Expected ActivityRecorded");
    }
}

#[test]
fn lancedb_embedded_uri_is_local_path() {
    let events = build_lancedb_embedded_journal("r");
    if let Some(Event::ActivityRecorded(a)) = &events[1].event {
        let v: serde_json::Value = serde_json::from_str(&a.input_json).unwrap();
        let uri = v["uri"].as_str().unwrap();
        assert!(uri.starts_with('/'), "expected local path, got {}", uri);
    } else {
        panic!("Expected ActivityRecorded");
    }
}

#[test]
fn lancedb_embedded_result_source_is_local() {
    let events = build_lancedb_embedded_journal("r");
    if let Some(Event::ActivityRecorded(a)) = &events[1].event {
        assert!(a.result_json.contains("\"source\":\"local\""));
    } else {
        panic!("Expected ActivityRecorded");
    }
}

#[test]
fn lancedb_embedded_no_network_uri_in_input() {
    let events = build_lancedb_embedded_journal("r");
    if let Some(Event::ActivityRecorded(a)) = &events[1].event {
        assert!(!a.input_json.contains("http"));
    } else {
        panic!("Expected ActivityRecorded");
    }
}

#[test]
fn lancedb_embedded_seq_monotonic() {
    let events = build_lancedb_embedded_journal("r");
    for (i, ev) in events.iter().enumerate() {
        assert_eq!(ev.seq, i as u64);
    }
}
