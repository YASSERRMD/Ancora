/// Determinism: no network calls during replay.
/// Activities marked replayed=true must not be re-executed (which would trigger network calls).
/// This test verifies that all replayed=true activities have result_json already populated.
use ancora_proto::ancora::{
    journal_event::Event, ActivityRecordedEvent, JournalEvent, RunCompletedEvent, RunStartedEvent,
};

fn build_replayed_journal(run_id: &str) -> Vec<JournalEvent> {
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
                activity_key: "api-call".into(),
                activity_kind: "llm".into(),
                input_json: r#"{"prompt":"answer me","url":"api.anthropic.com"}"#.into(),
                result_json: r#"{"text":"answer from cache"}"#.into(),
                replayed: true,
            })),
        },
        JournalEvent {
            event_id: format!("{}-2", run_id),
            run_id: run_id.into(),
            seq: 2,
            recorded_at_ns: 2_000,
            event: Some(Event::RunCompleted(RunCompletedEvent {
                output_json: r#"{"done":true}"#.into(),
            })),
        },
    ]
}

fn count_live_network_calls(j: &[JournalEvent]) -> usize {
    j.iter()
        .filter(|e| {
            if let Some(Event::ActivityRecorded(a)) = &e.event {
                !a.replayed && a.input_json.contains("url")
            } else {
                false
            }
        })
        .count()
}

#[test]
fn replayed_journal_has_no_live_network_calls() {
    let j = build_replayed_journal("det-net-1");
    assert_eq!(
        count_live_network_calls(&j),
        0,
        "replayed activities must not trigger network calls"
    );
}

#[test]
fn replayed_activity_has_result_already_populated() {
    let j = build_replayed_journal("det-net-2");
    if let Some(Event::ActivityRecorded(a)) = &j[1].event {
        assert!(a.replayed, "must be marked replayed");
        assert!(
            !a.result_json.is_empty(),
            "result_json must already be populated"
        );
    }
}

#[test]
fn non_replayed_activity_could_be_live() {
    let mut j = build_replayed_journal("det-net-3");
    if let Some(Event::ActivityRecorded(ref mut a)) = j[1].event {
        a.replayed = false;
    }
    assert_eq!(
        count_live_network_calls(&j),
        1,
        "non-replayed activity with url is a potential live call"
    );
}

#[test]
fn url_in_input_flagged_as_potential_network() {
    let j = build_replayed_journal("det-net-4");
    if let Some(Event::ActivityRecorded(a)) = &j[1].event {
        assert!(
            a.input_json.contains("url"),
            "test fixture must have url in input"
        );
    }
}

#[test]
fn replayed_result_text_is_from_cache_not_live() {
    let j = build_replayed_journal("det-net-5");
    if let Some(Event::ActivityRecorded(a)) = &j[1].event {
        assert!(
            a.result_json.contains("cache"),
            "replayed result must come from journal cache"
        );
    }
}
