/// Cross-language conformance: verifier scenario -- Rust.
use ancora_core::{
    journal::{JournalStore, MemoryStore},
    replay::replay_events,
    run::RunStatus,
};
use ancora_proto::ancora::{
    journal_event::Event, ActivityRecordedEvent, JournalEvent, RunCompletedEvent, RunStartedEvent,
};

fn build_verifier_journal(run_id: &str) -> Vec<JournalEvent> {
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
                activity_key: "drafter".into(),
                activity_kind: "agent-output".into(),
                input_json: r#"{"node":"drafter"}"#.into(),
                result_json: r#"{"draft":"xlang rust draft"}"#.into(),
                replayed: false,
            })),
        },
        JournalEvent {
            event_id: format!("{}-2", run_id),
            run_id: run_id.into(),
            seq: 2,
            recorded_at_ns: 2_000,
            event: Some(Event::ActivityRecorded(ActivityRecordedEvent {
                activity_key: "verifier".into(),
                activity_kind: "agent-output".into(),
                input_json: r#"{"node":"verifier","input":"xlang rust draft"}"#.into(),
                result_json: r#"{"verdict":"approved","score":1.0}"#.into(),
                replayed: false,
            })),
        },
        JournalEvent {
            event_id: format!("{}-3", run_id),
            run_id: run_id.into(),
            seq: 3,
            recorded_at_ns: 3_000,
            event: Some(Event::RunCompleted(RunCompletedEvent {
                output_json: r#"{"verdict":"approved"}"#.into(),
            })),
        },
    ]
}

#[test]
fn xlang_verifier_rust_completes() {
    let store = MemoryStore::new();
    let rid = "xlv-rust";
    for ev in &build_verifier_journal(rid) {
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
fn xlang_verifier_rust_has_two_activity_events() {
    let evs = build_verifier_journal("r");
    let act = evs
        .iter()
        .filter(|e| matches!(e.event, Some(Event::ActivityRecorded(_))))
        .count();
    assert_eq!(act, 2);
}
#[test]
fn xlang_verifier_rust_verdict_is_approved() {
    let evs = build_verifier_journal("r");
    if let Some(Event::RunCompleted(c)) = &evs.last().unwrap().event {
        assert!(c.output_json.contains("approved"));
    }
}
#[test]
fn xlang_verifier_rust_drafter_before_verifier() {
    let evs = build_verifier_journal("r");
    let keys: Vec<&str> = evs
        .iter()
        .filter_map(|e| {
            if let Some(Event::ActivityRecorded(a)) = &e.event {
                Some(a.activity_key.as_str())
            } else {
                None
            }
        })
        .collect();
    assert_eq!(keys, vec!["drafter", "verifier"]);
}
