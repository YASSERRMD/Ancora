/// Cross-language conformance: single agent scenario -- Rust binding.
/// Uses the same journal/replay infrastructure to prove the event contract.
use ancora_core::{
    journal::{JournalStore, MemoryStore},
    replay::replay_events,
    run::RunStatus,
};
use ancora_proto::ancora::{
    journal_event::Event, JournalEvent, RunCompletedEvent, RunStartedEvent,
};

fn xlang_single_agent_journal(run_id: &str) -> Vec<JournalEvent> {
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
            event: Some(Event::RunCompleted(RunCompletedEvent {
                output_json: r#"{"text":"xlang rust result"}"#.into(),
            })),
        },
    ]
}

#[test]
fn xlang_rust_single_agent_completes() {
    let store = MemoryStore::new();
    let run_id = "xlang-rust-001";
    for ev in &xlang_single_agent_journal(run_id) {
        store.append(run_id, ev.clone()).unwrap();
    }
    let state = replay_events(run_id, &store.read(run_id).unwrap()).unwrap();
    assert_eq!(state.run.status, RunStatus::Completed);
}

#[test]
fn xlang_rust_journal_has_started_as_first_event() {
    let events = xlang_single_agent_journal("r");
    assert!(matches!(events[0].event, Some(Event::RunStarted(_))));
}

#[test]
fn xlang_rust_journal_has_completed_as_last_event() {
    let events = xlang_single_agent_journal("r");
    assert!(matches!(
        events.last().unwrap().event,
        Some(Event::RunCompleted(_))
    ));
}

#[test]
fn xlang_rust_run_id_consistent_across_events() {
    let run_id = "xlang-rust-id";
    for ev in &xlang_single_agent_journal(run_id) {
        assert_eq!(ev.run_id, run_id);
    }
}

#[test]
fn xlang_rust_seq_monotonic_from_zero() {
    let events = xlang_single_agent_journal("r");
    for (i, ev) in events.iter().enumerate() {
        assert_eq!(ev.seq, i as u64);
    }
}

#[test]
fn xlang_rust_output_is_valid_json() {
    let events = xlang_single_agent_journal("r");
    if let Some(Event::RunCompleted(c)) = &events.last().unwrap().event {
        serde_json::from_str::<serde_json::Value>(&c.output_json).unwrap();
    } else {
        panic!("Expected RunCompleted");
    }
}
