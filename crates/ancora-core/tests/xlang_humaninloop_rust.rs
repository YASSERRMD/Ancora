/// Cross-language conformance: human-in-loop scenario -- Rust.
use ancora_core::{
    journal::{JournalStore, MemoryStore},
    replay::replay_events,
    run::RunStatus,
};
use ancora_proto::ancora::{
    journal_event::Event, HumanDecisionReceivedEvent, HumanDecisionRequestedEvent,
    JournalEvent, RunCompletedEvent, RunStartedEvent,
};

fn build_hil_journal(run_id: &str) -> Vec<JournalEvent> {
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
            event: Some(Event::HumanDecisionRequested(HumanDecisionRequestedEvent {
                prompt: "Please approve the draft".into(),
                options: vec!["approve".into(), "reject".into()],
                timeout_at_ns: 0,
            })),
        },
        JournalEvent {
            event_id: format!("{}-2", run_id),
            run_id: run_id.into(),
            seq: 2,
            recorded_at_ns: 2_000,
            event: Some(Event::HumanDecisionReceived(HumanDecisionReceivedEvent {
                decision: r#"{"approved":true}"#.into(),
            })),
        },
        JournalEvent {
            event_id: format!("{}-3", run_id),
            run_id: run_id.into(),
            seq: 3,
            recorded_at_ns: 3_000,
            event: Some(Event::RunCompleted(RunCompletedEvent {
                output_json: r#"{"result":"hil-ok"}"#.into(),
            })),
        },
    ]
}

#[test]
fn xlang_hil_rust_completes() {
    let store = MemoryStore::new();
    let rid = "xlh-rust";
    for ev in &build_hil_journal(rid) {
        store.append(rid, ev.clone()).unwrap();
    }
    assert_eq!(
        replay_events(rid, &store.read(rid).unwrap()).unwrap().run.status,
        RunStatus::Completed
    );
}

#[test]
fn xlang_hil_rust_decision_requested_before_received() {
    let evs = build_hil_journal("r");
    let kinds: Vec<&str> = evs.iter().filter_map(|e| match &e.event {
        Some(Event::HumanDecisionRequested(_)) => Some("requested"),
        Some(Event::HumanDecisionReceived(_)) => Some("received"),
        _ => None,
    }).collect();
    assert_eq!(kinds, vec!["requested", "received"]);
}

#[test]
fn xlang_hil_rust_decision_json_is_approved() {
    let evs = build_hil_journal("r");
    if let Some(Event::HumanDecisionReceived(h)) = &evs[2].event {
        assert!(h.decision.contains("true"));
    } else {
        panic!("Expected HumanDecisionReceived at index 2");
    }
}

#[test]
fn xlang_hil_rust_prompt_is_non_empty() {
    let evs = build_hil_journal("r");
    if let Some(Event::HumanDecisionRequested(h)) = &evs[1].event {
        assert!(!h.prompt.is_empty());
        assert!(!h.options.is_empty());
    } else {
        panic!("Expected HumanDecisionRequested at index 1");
    }
}

#[test]
fn xlang_hil_rust_seq_monotonic() {
    let evs = build_hil_journal("r");
    for (i, ev) in evs.iter().enumerate() {
        assert_eq!(ev.seq, i as u64);
    }
}
