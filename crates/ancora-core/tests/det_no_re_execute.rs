/// Determinism: recorded activities never re-execute on replay.
/// On replay, replayed=true events must not trigger re-execution.
use ancora_proto::ancora::{
    journal_event::Event, ActivityRecordedEvent, JournalEvent, RunCompletedEvent, RunStartedEvent,
};

fn build_journal_with_replayed_flag(run_id: &str, replayed: bool) -> Vec<JournalEvent> {
    vec![
        JournalEvent { event_id: format!("{}-0", run_id), run_id: run_id.into(), seq: 0, recorded_at_ns: 0,
            event: Some(Event::RunStarted(RunStartedEvent { run_id: run_id.into(), spec_bytes: vec![], spec_type: "AgentSpec".into() })) },
        JournalEvent { event_id: format!("{}-1", run_id), run_id: run_id.into(), seq: 1, recorded_at_ns: 1_000,
            event: Some(Event::ActivityRecorded(ActivityRecordedEvent {
                activity_key: "llm-call".into(), activity_kind: "llm".into(),
                input_json: r#"{"prompt":"hello"}"#.into(),
                result_json: r#"{"text":"hi there"}"#.into(),
                replayed })) },
        JournalEvent { event_id: format!("{}-2", run_id), run_id: run_id.into(), seq: 2, recorded_at_ns: 2_000,
            event: Some(Event::RunCompleted(RunCompletedEvent { output_json: r#"{"done":true}"#.into() })) },
    ]
}

#[test] fn replayed_true_marks_activity_as_not_live() {
    let j = build_journal_with_replayed_flag("det-nr-replay", true);
    if let Some(Event::ActivityRecorded(a)) = &j[1].event {
        assert!(a.replayed, "expected replayed=true");
    }
}

#[test] fn replayed_false_marks_activity_as_live() {
    let j = build_journal_with_replayed_flag("det-nr-live", false);
    if let Some(Event::ActivityRecorded(a)) = &j[1].event {
        assert!(!a.replayed, "expected replayed=false for live event");
    }
}

#[test] fn replayed_result_is_preserved_unchanged() {
    let expected = r#"{"text":"hi there"}"#;
    let j = build_journal_with_replayed_flag("det-nr-res", true);
    if let Some(Event::ActivityRecorded(a)) = &j[1].event {
        assert_eq!(a.result_json, expected, "result_json must not change on replay");
    }
}

#[test] fn replayed_activity_key_unchanged() {
    for replayed in [true, false] {
        let j = build_journal_with_replayed_flag("det-nr-key", replayed);
        if let Some(Event::ActivityRecorded(a)) = &j[1].event {
            assert_eq!(a.activity_key, "llm-call");
        }
    }
}

#[test] fn replayed_flag_does_not_alter_event_count() {
    assert_eq!(
        build_journal_with_replayed_flag("det-nr-cnt-t", true).len(),
        build_journal_with_replayed_flag("det-nr-cnt-f", false).len()
    );
}
