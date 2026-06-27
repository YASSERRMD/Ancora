/// Vector store conformance tests across three store archetypes (offline).
///
/// Validates that the journal representation of retrieval activities is
/// consistent whether the backing store is LanceDB, pgvector, or Qdrant.
/// All retrievals are represented as offline fixtures embedded in journal
/// ActivityRecordedEvents; no live store connections are made.
use ancora_core::{
    journal::{JournalStore, MemoryStore},
    journal_mask::{assert_structurally_equal, mask_events},
    replay::replay_events,
    run::RunStatus,
};
use ancora_proto::ancora::{
    journal_event::Event, ActivityRecordedEvent, JournalEvent, RunCompletedEvent, RunStartedEvent,
};

#[derive(Debug, Clone, Copy)]
enum StoreKind {
    LanceDb,
    PgVector,
    Qdrant,
}

impl StoreKind {
    fn label(&self) -> &'static str {
        match self {
            StoreKind::LanceDb => "lancedb",
            StoreKind::PgVector => "pgvector",
            StoreKind::Qdrant => "qdrant",
        }
    }
}

fn fixture_chunks(store: StoreKind) -> Vec<(String, f32)> {
    match store {
        StoreKind::LanceDb => vec![
            ("Chunk A from LanceDB".into(), 0.95),
            ("Chunk B from LanceDB".into(), 0.90),
        ],
        StoreKind::PgVector => vec![
            ("Chunk A from pgvector".into(), 0.93),
            ("Chunk B from pgvector".into(), 0.88),
        ],
        StoreKind::Qdrant => vec![
            ("Chunk A from Qdrant".into(), 0.94),
            ("Chunk B from Qdrant".into(), 0.89),
        ],
    }
}

fn retrieval_json(chunks: &[(String, f32)]) -> String {
    let items: Vec<String> = chunks
        .iter()
        .map(|(t, s)| format!(r#"{{"text":{},"score":{}}}"#, serde_json::to_string(t).unwrap(), s))
        .collect();
    format!("[{}]", items.join(","))
}

fn build_retrieval_journal(store_kind: StoreKind) -> Vec<JournalEvent> {
    let run_id = format!("{}-run", store_kind.label());
    let chunks = fixture_chunks(store_kind);
    let result = retrieval_json(&chunks);

    vec![
        JournalEvent {
            event_id: format!("{}-0", run_id),
            run_id: run_id.clone(),
            seq: 0,
            recorded_at_ns: 0,
            event: Some(Event::RunStarted(RunStartedEvent {
                run_id: run_id.clone(),
                spec_bytes: vec![],
                spec_type: "AgentSpec".into(),
            })),
        },
        JournalEvent {
            event_id: format!("{}-1", run_id),
            run_id: run_id.clone(),
            seq: 1,
            recorded_at_ns: 1_000_000,
            event: Some(Event::ActivityRecorded(ActivityRecordedEvent {
                activity_key: format!("retrieve-{}", store_kind.label()),
                activity_kind: "retrieval".into(),
                input_json: r#"{"query":"test","top_k":2}"#.into(),
                result_json: result,
                replayed: false,
            })),
        },
        JournalEvent {
            event_id: format!("{}-2", run_id),
            run_id: run_id.clone(),
            seq: 2,
            recorded_at_ns: 2_000_000,
            event: Some(Event::RunCompleted(RunCompletedEvent {
                output_json: r#"{"answer":"retrieved"}"#.into(),
            })),
        },
    ]
}

#[test]
fn lancedb_retrieval_journal_replays_to_completed() {
    let store = MemoryStore::new();
    let events = build_retrieval_journal(StoreKind::LanceDb);
    let run_id = "lancedb-run";

    for ev in &events {
        store.append(run_id, ev.clone()).unwrap();
    }

    let loaded = store.read(run_id).unwrap();
    let state = replay_events(run_id, &loaded).unwrap();
    assert_eq!(state.run.status, RunStatus::Completed);
}

#[test]
fn pgvector_retrieval_journal_replays_to_completed() {
    let store = MemoryStore::new();
    let events = build_retrieval_journal(StoreKind::PgVector);
    let run_id = "pgvector-run";

    for ev in &events {
        store.append(run_id, ev.clone()).unwrap();
    }

    let loaded = store.read(run_id).unwrap();
    let state = replay_events(run_id, &loaded).unwrap();
    assert_eq!(state.run.status, RunStatus::Completed);
}

#[test]
fn qdrant_retrieval_journal_replays_to_completed() {
    let store = MemoryStore::new();
    let events = build_retrieval_journal(StoreKind::Qdrant);
    let run_id = "qdrant-run";

    for ev in &events {
        store.append(run_id, ev.clone()).unwrap();
    }

    let loaded = store.read(run_id).unwrap();
    let state = replay_events(run_id, &loaded).unwrap();
    assert_eq!(state.run.status, RunStatus::Completed);
}

#[test]
fn three_store_journals_are_structurally_equal_after_masking() {
    let j1 = build_retrieval_journal(StoreKind::LanceDb);
    let j2 = build_retrieval_journal(StoreKind::PgVector);
    let j3 = build_retrieval_journal(StoreKind::Qdrant);

    let m1 = mask_events(&j1);
    let m2 = mask_events(&j2);
    let m3 = mask_events(&j3);

    assert_structurally_equal(&m1, &m2)
        .expect("lancedb and pgvector journals must be structurally equal");
    assert_structurally_equal(&m2, &m3)
        .expect("pgvector and qdrant journals must be structurally equal");
}

#[test]
fn each_store_returns_exactly_two_chunks() {
    for kind in [StoreKind::LanceDb, StoreKind::PgVector, StoreKind::Qdrant] {
        let chunks = fixture_chunks(kind);
        assert_eq!(chunks.len(), 2, "{} must return 2 chunks", kind.label());
    }
}

#[test]
fn retrieval_json_parses_to_array_of_length_two() {
    for kind in [StoreKind::LanceDb, StoreKind::PgVector, StoreKind::Qdrant] {
        let json = retrieval_json(&fixture_chunks(kind));
        let v: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(v.as_array().unwrap().len(), 2, "{} JSON array must have 2 elements", kind.label());
    }
}

#[test]
fn store_labels_are_distinct() {
    let labels = [StoreKind::LanceDb.label(), StoreKind::PgVector.label(), StoreKind::Qdrant.label()];
    let unique: std::collections::HashSet<&str> = labels.iter().copied().collect();
    assert_eq!(unique.len(), 3);
}

#[test]
fn retrieval_activity_kind_is_retrieval_for_all_stores() {
    for kind in [StoreKind::LanceDb, StoreKind::PgVector, StoreKind::Qdrant] {
        let events = build_retrieval_journal(kind);
        let retrieval: Vec<_> = events
            .iter()
            .filter(|e| matches!(&e.event, Some(Event::ActivityRecorded(a)) if a.activity_kind == "retrieval"))
            .collect();
        assert_eq!(retrieval.len(), 1, "{} must have exactly one retrieval activity", kind.label());
    }
}
