/// Migration script: read from one store, write to another.
use ancora_core::{
    journal::{JournalStore, MemoryStore},
    replay::replay_events,
    run::RunStatus,
};
use ancora_proto::ancora::{
    journal_event::Event, ActivityRecordedEvent, JournalEvent, RunCompletedEvent, RunStartedEvent,
};

fn build_migration_journal(
    run_id: &str,
    source: &str,
    dest: &str,
    chunks_migrated: u32,
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
                activity_key: format!("migrate-{}-to-{}", source, dest),
                activity_kind: "migration".into(),
                input_json: format!(
                    r#"{{"source_store":"{}","dest_store":"{}","batch_size":100,"dry_run":false}}"#,
                    source, dest
                ),
                result_json: format!(
                    r#"{{"chunks_read":{},"chunks_written":{},"errors":0,"skipped":0,"source":"{}","dest":"{}"}}"#,
                    chunks_migrated, chunks_migrated, source, dest
                ),
                replayed: false,
            })),
        },
        JournalEvent {
            event_id: format!("{}-2", run_id),
            run_id: run_id.to_string(),
            seq: 2,
            recorded_at_ns: 2_000,
            event: Some(Event::RunCompleted(RunCompletedEvent {
                output_json: format!(r#"{{"migrated":{},"status":"success"}}"#, chunks_migrated),
            })),
        },
    ]
}

#[test]
fn migration_sqlite_to_pgvector_replays() {
    let store = MemoryStore::new();
    let run_id = "mig-sq-pg";
    for ev in &build_migration_journal(run_id, "sqlite", "pgvector", 42) {
        store.append(run_id, ev.clone()).unwrap();
    }
    let state = replay_events(run_id, &store.read(run_id).unwrap()).unwrap();
    assert_eq!(state.run.status, RunStatus::Completed);
}

#[test]
fn migration_inmemory_to_qdrant_replays() {
    let store = MemoryStore::new();
    let run_id = "mig-im-qd";
    for ev in &build_migration_journal(run_id, "inmemory", "qdrant", 10) {
        store.append(run_id, ev.clone()).unwrap();
    }
    let state = replay_events(run_id, &store.read(run_id).unwrap()).unwrap();
    assert_eq!(state.run.status, RunStatus::Completed);
}

#[test]
fn migration_activity_kind_is_migration() {
    let events = build_migration_journal("r", "sqlite", "pgvector", 5);
    if let Some(Event::ActivityRecorded(a)) = &events[1].event {
        assert_eq!(a.activity_kind, "migration");
    } else {
        panic!("Expected ActivityRecorded");
    }
}

#[test]
fn migration_result_reads_and_writes_equal() {
    let events = build_migration_journal("r", "sqlite", "pgvector", 42);
    if let Some(Event::ActivityRecorded(a)) = &events[1].event {
        let v: serde_json::Value = serde_json::from_str(&a.result_json).unwrap();
        assert_eq!(v["chunks_read"], v["chunks_written"]);
    } else {
        panic!("Expected ActivityRecorded");
    }
}

#[test]
fn migration_result_has_zero_errors() {
    let events = build_migration_journal("r", "sqlite", "pgvector", 20);
    if let Some(Event::ActivityRecorded(a)) = &events[1].event {
        let v: serde_json::Value = serde_json::from_str(&a.result_json).unwrap();
        assert_eq!(v["errors"], 0);
    } else {
        panic!("Expected ActivityRecorded");
    }
}

#[test]
fn migration_result_source_and_dest_match_input() {
    let events = build_migration_journal("r", "sqlite", "pgvector", 5);
    if let Some(Event::ActivityRecorded(a)) = &events[1].event {
        let inp: serde_json::Value = serde_json::from_str(&a.input_json).unwrap();
        let res: serde_json::Value = serde_json::from_str(&a.result_json).unwrap();
        assert_eq!(inp["source_store"], res["source"]);
        assert_eq!(inp["dest_store"], res["dest"]);
    } else {
        panic!("Expected ActivityRecorded");
    }
}

#[test]
fn migration_seq_monotonic() {
    let events = build_migration_journal("r", "sqlite", "pgvector", 1);
    for (i, ev) in events.iter().enumerate() {
        assert_eq!(ev.seq, i as u64);
    }
}
