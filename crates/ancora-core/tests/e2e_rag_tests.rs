/// End-to-end RAG (Retrieval-Augmented Generation) tests (offline).
///
/// These tests validate the journal-level representation of a RAG flow:
/// a retrieval activity followed by an LLM generation activity. No live
/// vector store calls are made; the retrieval results are fixture data.
use ancora_core::{
    journal::{JournalStore, MemoryStore},
    replay::replay_events,
    run::RunStatus,
};
use ancora_proto::ancora::{
    journal_event::Event, ActivityRecordedEvent, JournalEvent, NodeEnteredEvent,
    NodeExitedEvent, RunCompletedEvent, RunStartedEvent,
};

fn ev(seq: u64, run_id: &str, event: Event) -> JournalEvent {
    JournalEvent {
        event_id: format!("{}-{}", run_id, seq),
        run_id: run_id.to_owned(),
        seq,
        recorded_at_ns: (seq * 1_000_000) as i64,
        event: Some(event),
    }
}

const FIXTURE_CHUNKS: &[(&str, f32)] = &[
    ("Rust memory model: ownership and borrowing.", 0.91),
    ("Lifetimes prevent dangling references.", 0.87),
    ("The borrow checker enforces these rules at compile time.", 0.83),
];

fn rag_retrieval_json(chunks: &[(&str, f32)]) -> String {
    let items: Vec<String> = chunks
        .iter()
        .map(|(text, score)| format!(r#"{{"text":{},"score":{}}}"#, serde_json::to_string(text).unwrap(), score))
        .collect();
    format!("[{}]", items.join(","))
}

fn build_rag_journal(run_id: &str) -> Vec<JournalEvent> {
    let retrieval = rag_retrieval_json(FIXTURE_CHUNKS);
    let generation = r#"{"answer":"Rust uses ownership and borrowing to manage memory safely."}"#;

    vec![
        ev(0, run_id, Event::RunStarted(RunStartedEvent {
            run_id: run_id.to_owned(),
            spec_bytes: vec![],
            spec_type: "AgentSpec".into(),
        })),
        ev(1, run_id, Event::NodeEntered(NodeEnteredEvent {
            node_id: "rag-node".into(),
            node_kind: "agent".into(),
        })),
        ev(2, run_id, Event::ActivityRecorded(ActivityRecordedEvent {
            activity_key: "vector-retrieve-1".into(),
            activity_kind: "retrieval".into(),
            input_json: r#"{"query":"rust memory management","top_k":3}"#.into(),
            result_json: retrieval,
            replayed: false,
        })),
        ev(3, run_id, Event::ActivityRecorded(ActivityRecordedEvent {
            activity_key: "llm-generate-1".into(),
            activity_kind: "llm".into(),
            input_json: r#"{"context":"...","question":"rust memory management"}"#.into(),
            result_json: generation.into(),
            replayed: false,
        })),
        ev(4, run_id, Event::NodeExited(NodeExitedEvent {
            node_id: "rag-node".into(),
            success: true,
        })),
        ev(5, run_id, Event::RunCompleted(RunCompletedEvent {
            output_json: generation.into(),
        })),
    ]
}

#[test]
fn rag_e2e_journal_replays_to_completed() {
    let run_id = "e2e-rag-1";
    let store = MemoryStore::new();

    for event in build_rag_journal(run_id) {
        store.append(run_id, event).unwrap();
    }

    let events = store.read(run_id).unwrap();
    let state = replay_events(run_id, &events).unwrap();
    assert_eq!(state.run.status, RunStatus::Completed);
}

#[test]
fn rag_e2e_retrieval_activity_key_is_first() {
    let run_id = "e2e-rag-order";
    let store = MemoryStore::new();

    for event in build_rag_journal(run_id) {
        store.append(run_id, event).unwrap();
    }

    let events = store.read(run_id).unwrap();
    let state = replay_events(run_id, &events).unwrap();
    assert_eq!(state.activity_keys[0], "vector-retrieve-1");
    assert_eq!(state.activity_keys[1], "llm-generate-1");
}

#[test]
fn rag_e2e_two_activity_keys_recorded() {
    let run_id = "e2e-rag-keys";
    let store = MemoryStore::new();

    for event in build_rag_journal(run_id) {
        store.append(run_id, event).unwrap();
    }

    let events = store.read(run_id).unwrap();
    let state = replay_events(run_id, &events).unwrap();
    assert_eq!(state.activity_keys.len(), 2);
}

#[test]
fn rag_retrieval_fixture_has_three_chunks() {
    assert_eq!(FIXTURE_CHUNKS.len(), 3);
}

#[test]
fn rag_retrieval_fixture_scores_descend() {
    let scores: Vec<f32> = FIXTURE_CHUNKS.iter().map(|(_, s)| *s).collect();
    for w in scores.windows(2) {
        assert!(w[0] >= w[1], "scores must be in descending order");
    }
}

#[test]
fn rag_retrieval_json_parses_as_valid_json_array() {
    let json = rag_retrieval_json(FIXTURE_CHUNKS);
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert!(parsed.is_array());
    assert_eq!(parsed.as_array().unwrap().len(), 3);
}

#[test]
fn rag_e2e_replay_is_deterministic() {
    let run_id = "e2e-rag-idem";
    let events = build_rag_journal(run_id);

    let a = replay_events(run_id, &events).unwrap();
    let b = replay_events(run_id, &events).unwrap();
    assert_eq!(a.run.status, b.run.status);
    assert_eq!(a.activity_keys, b.activity_keys);
}

#[test]
fn rag_e2e_retrieval_activity_kind_is_retrieval() {
    let run_id = "e2e-rag-kind";
    let events = build_rag_journal(run_id);

    let retrieval_events: Vec<&JournalEvent> = events
        .iter()
        .filter(|e| {
            if let Some(Event::ActivityRecorded(a)) = &e.event {
                a.activity_kind == "retrieval"
            } else {
                false
            }
        })
        .collect();

    assert_eq!(retrieval_events.len(), 1, "exactly one retrieval activity");
}

#[test]
fn rag_e2e_generation_output_is_valid_json() {
    let run_id = "e2e-rag-json";
    let events = build_rag_journal(run_id);

    let completed_output = events
        .iter()
        .find_map(|e| {
            if let Some(Event::RunCompleted(c)) = &e.event {
                Some(c.output_json.clone())
            } else {
                None
            }
        })
        .unwrap();

    let parsed: serde_json::Value = serde_json::from_str(&completed_output).unwrap();
    assert!(parsed["answer"].is_string());
}

#[test]
fn rag_e2e_journal_has_six_events() {
    let events = build_rag_journal("e2e-rag-count");
    assert_eq!(events.len(), 6, "started+entered+retrieve+generate+exited+completed = 6");
}
