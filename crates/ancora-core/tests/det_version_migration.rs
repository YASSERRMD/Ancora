/// Determinism: journal version migration replay.
/// When journal schema version increments, old journals must still replay correctly
/// after a migration step that normalises the event format.
use ancora_core::{
    journal::{JournalStore, MemoryStore},
    replay::replay_events,
    run::RunStatus,
};
use ancora_proto::ancora::{
    journal_event::Event, ActivityRecordedEvent, JournalEvent, RunCompletedEvent, RunStartedEvent,
};
use serde_json::Value;

fn v1_journal(run_id: &str) -> Vec<JournalEvent> {
    vec![
        JournalEvent {
            event_id: format!("{}-0", run_id),
            run_id: run_id.into(),
            seq: 0,
            recorded_at_ns: 0,
            event: Some(Event::RunStarted(RunStartedEvent {
                run_id: run_id.into(),
                spec_bytes: vec![],
                spec_type: "AgentSpec".into(),
            })),
        },
        JournalEvent {
            event_id: format!("{}-1", run_id),
            run_id: run_id.into(),
            seq: 1,
            recorded_at_ns: 1_000,
            event: Some(Event::ActivityRecorded(ActivityRecordedEvent {
                activity_key: "search".into(),
                activity_kind: "retrieval".into(),
                input_json: r#"{"q":"hello"}"#.into(),
                result_json: r#"{"docs":["doc1","doc2"]}"#.into(),
                replayed: false,
            })),
        },
        JournalEvent {
            event_id: format!("{}-2", run_id),
            run_id: run_id.into(),
            seq: 2,
            recorded_at_ns: 2_000,
            event: Some(Event::RunCompleted(RunCompletedEvent {
                output_json: r#"{"ok":true}"#.into(),
            })),
        },
    ]
}

fn migrate_v1_to_v2(event: &JournalEvent) -> JournalEvent {
    let mut migrated = event.clone();
    if let Some(Event::ActivityRecorded(ref mut a)) = migrated.event {
        let mut v: Value = serde_json::from_str(&a.input_json).unwrap_or_default();
        if v.get("q").is_some() {
            v["query"] = v["q"].clone();
            v.as_object_mut().unwrap().remove("q");
            a.input_json = serde_json::to_string(&v).unwrap();
        }
    }
    migrated
}

#[test]
fn v1_journal_replays_to_completed_without_migration() {
    let store = MemoryStore::new();
    let rid = "det-v1-nomig";
    for ev in &v1_journal(rid) {
        store.append(rid, ev.clone()).unwrap();
    }
    assert_eq!(
        replay_events(rid, &store.read(rid).unwrap())
            .unwrap()
            .run
            .status,
        RunStatus::Completed
    );
}

#[test]
fn v1_to_v2_migration_changes_q_to_query() {
    let j = v1_journal("det-mig-q");
    let migrated: Vec<JournalEvent> = j.iter().map(migrate_v1_to_v2).collect();
    if let Some(Event::ActivityRecorded(a)) = &migrated[1].event {
        let v: Value = serde_json::from_str(&a.input_json).unwrap();
        assert!(
            v.get("query").is_some(),
            "expected query field after migration"
        );
        assert!(v.get("q").is_none(), "old q field must be removed");
    }
}

#[test]
fn migrated_v1_journal_still_replays_to_completed() {
    let j = v1_journal("det-v2-mig");
    let migrated: Vec<JournalEvent> = j.iter().map(migrate_v1_to_v2).collect();
    let store = MemoryStore::new();
    let rid = "det-v2-mig";
    for ev in &migrated {
        store.append(rid, ev.clone()).unwrap();
    }
    assert_eq!(
        replay_events(rid, &store.read(rid).unwrap())
            .unwrap()
            .run
            .status,
        RunStatus::Completed
    );
}

#[test]
fn migration_preserves_event_count() {
    let j = v1_journal("det-mig-cnt");
    let m: Vec<JournalEvent> = j.iter().map(migrate_v1_to_v2).collect();
    assert_eq!(j.len(), m.len());
}

#[test]
fn migration_preserves_seq_numbers() {
    let j = v1_journal("det-mig-seq");
    let m: Vec<JournalEvent> = j.iter().map(migrate_v1_to_v2).collect();
    for (orig, mig) in j.iter().zip(m.iter()) {
        assert_eq!(orig.seq, mig.seq);
    }
}
