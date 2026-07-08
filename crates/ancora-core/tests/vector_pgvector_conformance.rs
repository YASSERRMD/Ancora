/// pgvector conformance -- offline fixture, no live Postgres.
use ancora_core::{
    journal::{JournalStore, MemoryStore},
    replay::replay_events,
    run::RunStatus,
};
use ancora_proto::ancora::{
    journal_event::Event, ActivityRecordedEvent, JournalEvent, RunCompletedEvent, RunStartedEvent,
};

fn build_pg_journal(run_id: &str) -> Vec<JournalEvent> {
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
                activity_key: "retrieve-pgvector".into(),
                activity_kind: "retrieval".into(),
                input_json: r#"{"query":"pg docs","top_k":3,"index_type":"ivfflat"}"#.into(),
                result_json: r#"[{"text":"PG chunk A","score":0.96},{"text":"PG chunk B","score":0.91},{"text":"PG chunk C","score":0.84}]"#.into(),
                replayed: false,
            })),
        },
        JournalEvent {
            event_id: format!("{}-2", run_id),
            run_id: run_id.to_string(),
            seq: 2,
            recorded_at_ns: 2_000,
            event: Some(Event::RunCompleted(RunCompletedEvent {
                output_json: r#"{"answer":"pg-ok"}"#.into(),
            })),
        },
    ]
}

#[test]
fn pgvector_journal_replays_to_completed() {
    let store = MemoryStore::new();
    let run_id = "pg-run-151";
    for ev in &build_pg_journal(run_id) {
        store.append(run_id, ev.clone()).unwrap();
    }
    let state = replay_events(run_id, &store.read(run_id).unwrap()).unwrap();
    assert_eq!(state.run.status, RunStatus::Completed);
}

#[test]
fn pgvector_activity_key_contains_pgvector() {
    let events = build_pg_journal("r");
    if let Some(Event::ActivityRecorded(a)) = &events[1].event {
        assert!(a.activity_key.contains("pgvector"));
    } else {
        panic!("Expected ActivityRecorded");
    }
}

#[test]
fn pgvector_result_has_three_chunks() {
    let events = build_pg_journal("r");
    if let Some(Event::ActivityRecorded(a)) = &events[1].event {
        let v: serde_json::Value = serde_json::from_str(&a.result_json).unwrap();
        assert_eq!(v.as_array().unwrap().len(), 3);
    } else {
        panic!("Expected ActivityRecorded");
    }
}

#[test]
fn pgvector_input_json_has_index_type() {
    let events = build_pg_journal("r");
    if let Some(Event::ActivityRecorded(a)) = &events[1].event {
        assert!(a.input_json.contains("ivfflat"));
    } else {
        panic!("Expected ActivityRecorded");
    }
}

#[test]
fn pgvector_seq_monotonic() {
    let events = build_pg_journal("r");
    for (i, ev) in events.iter().enumerate() {
        assert_eq!(ev.seq, i as u64);
    }
}

#[test]
fn pgvector_run_ids_match() {
    let run_id = "pg-match";
    for ev in &build_pg_journal(run_id) {
        assert_eq!(ev.run_id, run_id);
    }
}
