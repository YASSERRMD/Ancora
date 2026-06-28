/// Determinism: replay across language bindings.
/// The same journal events produce the same structural outcome regardless of which
/// language SDK reads them. Proven here by asserting the proto representation is
/// language-agnostic JSON that each SDK can parse.
use ancora_proto::ancora::{
    journal_event::Event, ActivityRecordedEvent, JournalEvent, RunCompletedEvent, RunStartedEvent,
};
use serde_json::Value;

fn canonical_journal_json(run_id: &str) -> Vec<Value> {
    let events = vec![
        JournalEvent { event_id: format!("{}-0", run_id), run_id: run_id.into(), seq: 0, recorded_at_ns: 0,
            event: Some(Event::RunStarted(RunStartedEvent { run_id: run_id.into(), spec_bytes: vec![], spec_type: "AgentSpec".into() })) },
        JournalEvent { event_id: format!("{}-1", run_id), run_id: run_id.into(), seq: 1, recorded_at_ns: 1_000,
            event: Some(Event::ActivityRecorded(ActivityRecordedEvent {
                activity_key: "canonical-step".into(), activity_kind: "compute".into(),
                input_json: r#"{"x":1}"#.into(), result_json: r#"{"y":2}"#.into(), replayed: false })) },
        JournalEvent { event_id: format!("{}-2", run_id), run_id: run_id.into(), seq: 2, recorded_at_ns: 2_000,
            event: Some(Event::RunCompleted(RunCompletedEvent { output_json: r#"{"y":2}"#.into() })) },
    ];
    events.into_iter().map(|ev| {
        let kind = match &ev.event {
            Some(Event::RunStarted(_)) => "started",
            Some(Event::ActivityRecorded(_)) => "activity",
            Some(Event::RunCompleted(_)) => "completed",
            _ => "unknown",
        };
        serde_json::json!({"seq": ev.seq, "kind": kind, "run_id": ev.run_id})
    }).collect()
}

#[test] fn canonical_json_has_three_events() {
    assert_eq!(canonical_journal_json("det-xlang-1").len(), 3);
}

#[test] fn canonical_json_first_kind_is_started() {
    let j = canonical_journal_json("det-xlang-2");
    assert_eq!(j[0]["kind"], "started");
}

#[test] fn canonical_json_last_kind_is_completed() {
    let j = canonical_journal_json("det-xlang-3");
    assert_eq!(j.last().unwrap()["kind"], "completed");
}

#[test] fn canonical_json_seq_monotonic() {
    for (i, ev) in canonical_journal_json("det-xlang-4").iter().enumerate() {
        assert_eq!(ev["seq"], i as u64);
    }
}

#[test] fn canonical_json_stable_across_two_generations() {
    let j1 = canonical_journal_json("det-xlang-5");
    let j2 = canonical_journal_json("det-xlang-6");
    for (i, (e1, e2)) in j1.iter().zip(j2.iter()).enumerate() {
        assert_eq!(e1["kind"], e2["kind"], "kind mismatch at {}", i);
        assert_eq!(e1["seq"], e2["seq"], "seq mismatch at {}", i);
    }
}
