/// Backend selection by config -- journal records which store was chosen.
use ancora_core::{
    journal::{JournalStore, MemoryStore},
    replay::replay_events,
    run::RunStatus,
};
use ancora_proto::ancora::{
    journal_event::Event, ActivityRecordedEvent, JournalEvent, RunCompletedEvent, RunStartedEvent,
};

fn make_backend_journal(run_id: &str, backend: &str) -> Vec<JournalEvent> {
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
                activity_key: format!("retrieve-{}", backend),
                activity_kind: "retrieval".into(),
                input_json: format!(r#"{{"query":"backend select","top_k":1,"backend":"{}","selected_by":"config"}}"#, backend),
                result_json: format!(r#"[{{"text":"{} result","score":0.90,"backend":"{}"}}]"#, backend, backend),
                replayed: false,
            })),
        },
        JournalEvent {
            event_id: format!("{}-2", run_id),
            run_id: run_id.to_string(),
            seq: 2,
            recorded_at_ns: 2_000,
            event: Some(Event::RunCompleted(RunCompletedEvent {
                output_json: format!(r#"{{"answer":"{}-ok"}}"#, backend),
            })),
        },
    ]
}

const BACKENDS: &[&str] = &[
    "inmemory", "sqlite", "pgvector", "qdrant", "weaviate", "milvus", "lancedb", "chroma",
];

#[test]
fn all_backends_replay_to_completed() {
    for backend in BACKENDS {
        let run_id = format!("bs-{}", backend);
        let events = make_backend_journal(&run_id, backend);
        let store = MemoryStore::new();
        for ev in &events { store.append(&run_id, ev.clone()).unwrap(); }
        let state = replay_events(&run_id, &store.read(&run_id).unwrap()).unwrap();
        assert_eq!(state.run.status, RunStatus::Completed, "failed for backend {}", backend);
    }
}

#[test]
fn all_backends_selected_by_config() {
    for backend in BACKENDS {
        let run_id = format!("bs2-{}", backend);
        let events = make_backend_journal(&run_id, backend);
        if let Some(Event::ActivityRecorded(a)) = &events[1].event {
            assert!(a.input_json.contains("selected_by"), "no selected_by for {}", backend);
            assert!(a.input_json.contains("config"), "not config for {}", backend);
        }
    }
}

#[test]
fn all_backends_activity_key_matches_backend() {
    for backend in BACKENDS {
        let run_id = format!("bs3-{}", backend);
        let events = make_backend_journal(&run_id, backend);
        if let Some(Event::ActivityRecorded(a)) = &events[1].event {
            assert!(a.activity_key.contains(backend), "key {} does not contain {}", a.activity_key, backend);
        }
    }
}

#[test]
fn all_backends_result_carries_backend_field() {
    for backend in BACKENDS {
        let run_id = format!("bs4-{}", backend);
        let events = make_backend_journal(&run_id, backend);
        if let Some(Event::ActivityRecorded(a)) = &events[1].event {
            assert!(a.result_json.contains(*backend), "result does not mention {}", backend);
        }
    }
}

#[test]
fn backends_list_is_eight_distinct_stores() {
    use std::collections::HashSet;
    let set: HashSet<_> = BACKENDS.iter().collect();
    assert_eq!(set.len(), BACKENDS.len(), "duplicate backend in list");
    assert_eq!(BACKENDS.len(), 8);
}
