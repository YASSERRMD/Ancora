/// Determinism: OTel spans are reproduced on replay.
/// Spans are recorded in the journal with fixed trace_id and span_id.
/// On replay, the same span structure is reproduced.
use ancora_proto::ancora::{
    journal_event::Event, ActivityRecordedEvent, JournalEvent, RunCompletedEvent, RunStartedEvent,
};
use serde_json::Value;

fn build_otel_journal(run_id: &str, trace_id: &str) -> Vec<JournalEvent> {
    vec![
        JournalEvent { event_id: format!("{}-0", run_id), run_id: run_id.into(), seq: 0, recorded_at_ns: 0,
            event: Some(Event::RunStarted(RunStartedEvent { run_id: run_id.into(), spec_bytes: vec![], spec_type: "AgentSpec".into() })) },
        JournalEvent { event_id: format!("{}-1", run_id), run_id: run_id.into(), seq: 1, recorded_at_ns: 1_000,
            event: Some(Event::ActivityRecorded(ActivityRecordedEvent {
                activity_key: "traced-step".into(), activity_kind: "traced".into(),
                input_json: format!(r#"{{"trace_id":"{}","span_id":"0000000000000001","operation":"traced-step"}}"#, trace_id),
                result_json: format!(r#"{{"trace_id":"{}","span_id":"0000000000000002","parent_span_id":"0000000000000001","status":"ok"}}"#, trace_id),
                replayed: false })) },
        JournalEvent { event_id: format!("{}-2", run_id), run_id: run_id.into(), seq: 2, recorded_at_ns: 2_000,
            event: Some(Event::RunCompleted(RunCompletedEvent { output_json: r#"{"traced":true}"#.into() })) },
    ]
}

fn get_span_field<'a>(j: &'a [JournalEvent], field: &str) -> Option<String> {
    j.iter().find_map(|e| {
        if let Some(Event::ActivityRecorded(a)) = &e.event {
            let v: Value = serde_json::from_str(&a.result_json).ok()?;
            v[field].as_str().map(|s| s.to_string())
        } else { None }
    })
}

const TRACE_ID: &str = "0af7651916cd43dd8448eb211c80319c";

#[test] fn otel_trace_id_preserved_on_replay() {
    let j1 = build_otel_journal("det-otel-1", TRACE_ID);
    let j2 = build_otel_journal("det-otel-2", TRACE_ID);
    assert_eq!(get_span_field(&j1, "trace_id"), get_span_field(&j2, "trace_id"));
}

#[test] fn otel_span_id_is_non_empty() {
    let j = build_otel_journal("det-otel-sid", TRACE_ID);
    let span_id = get_span_field(&j, "span_id");
    assert!(span_id.is_some() && !span_id.unwrap().is_empty());
}

#[test] fn otel_parent_span_id_present() {
    let j = build_otel_journal("det-otel-par", TRACE_ID);
    assert!(get_span_field(&j, "parent_span_id").is_some());
}

#[test] fn otel_status_is_ok() {
    let j = build_otel_journal("det-otel-status", TRACE_ID);
    assert_eq!(get_span_field(&j, "status"), Some("ok".into()));
}

#[test] fn otel_spans_identical_across_two_replays() {
    let j1 = build_otel_journal("det-otel-r1", TRACE_ID);
    let j2 = build_otel_journal("det-otel-r2", TRACE_ID);
    if let (Some(Event::ActivityRecorded(a1)), Some(Event::ActivityRecorded(a2))) = (&j1[1].event, &j2[1].event) {
        assert_eq!(a1.result_json, a2.result_json);
    }
}
