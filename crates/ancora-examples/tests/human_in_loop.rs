use ancora_core::journal::{JournalStore, MemoryStore};
use ancora_core::run::{Run, RunStatus};
use ancora_proto::ancora::{
    journal_event::Event, HumanDecisionReceivedEvent, HumanDecisionRequestedEvent, JournalEvent,
    RunStartedEvent,
};

fn make_event(run_id: &str, seq: u64, event: Event) -> JournalEvent {
    JournalEvent {
        event_id: format!("{run_id}-{seq}"),
        run_id: run_id.to_string(),
        seq,
        recorded_at_ns: seq as i64 * 1_000_000,
        event: Some(event),
    }
}

#[test]
fn human_decision_requested_event_round_trips() {
    let store = MemoryStore::new();
    let run_id = "hitl-run-1";

    store
        .append(
            run_id,
            make_event(
                run_id,
                0,
                Event::RunStarted(RunStartedEvent {
                    run_id: run_id.to_string(),
                    spec_bytes: vec![],
                    spec_type: "AgentSpec".to_string(),
                }),
            ),
        )
        .unwrap();

    store
        .append(
            run_id,
            make_event(
                run_id,
                1,
                Event::HumanDecisionRequested(HumanDecisionRequestedEvent {
                    prompt: "Approve this action?".to_string(),
                    options: vec!["yes".to_string(), "no".to_string()],
                    timeout_at_ns: 0,
                }),
            ),
        )
        .unwrap();

    let events = store.read(run_id).unwrap();
    assert_eq!(2, events.len());
    assert!(matches!(
        events[1].event,
        Some(Event::HumanDecisionRequested(_))
    ));
}

#[test]
fn human_decision_received_event_round_trips() {
    let store = MemoryStore::new();
    let run_id = "hitl-run-2";

    store
        .append(
            run_id,
            make_event(
                run_id,
                0,
                Event::HumanDecisionReceived(HumanDecisionReceivedEvent {
                    decision: "approved".to_string(),
                }),
            ),
        )
        .unwrap();

    let events = store.read(run_id).unwrap();
    if let Some(Event::HumanDecisionReceived(ref ev)) = events[0].event {
        assert_eq!("approved", ev.decision);
    } else {
        panic!("expected HumanDecisionReceived");
    }
}

#[test]
fn run_can_be_paused_by_staying_in_running() {
    let mut run = Run::new("hitl-run-3");
    run.transition(RunStatus::Running).unwrap();
    assert_eq!(RunStatus::Running, run.status);
}

#[test]
fn resume_decision_can_contain_arbitrary_json() {
    let decision_json = r#"{"action":"approve","reason":"looks good"}"#;
    let ev = HumanDecisionReceivedEvent {
        decision: decision_json.to_string(),
    };
    let parsed: serde_json::Value = serde_json::from_str(&ev.decision).unwrap();
    assert_eq!("approve", parsed["action"].as_str().unwrap());
}
