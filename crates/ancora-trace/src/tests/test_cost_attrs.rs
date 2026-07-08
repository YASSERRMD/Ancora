/// Tests: cost attributes match journal entries.
use crate::genai_attrs::{
    self, ANCORA_COST_USD, GEN_AI_USAGE_INPUT_TOKENS, GEN_AI_USAGE_OUTPUT_TOKENS,
};
use crate::journal::{journal_event_to_span, JournalEvent, JournalEventKind, JournalMetadata};
use crate::span::{SpanId, TraceId};

fn make_llm_event(cost: f64, input_tokens: i64, output_tokens: i64) -> JournalEvent {
    JournalEvent {
        kind: JournalEventKind::LlmCallFinished,
        timestamp_ns: 1_000,
        trace_id: TraceId("t1".into()),
        span_id: SpanId("s1".into()),
        parent_id: None,
        name: "llm-call".into(),
        metadata: JournalMetadata {
            cost_usd: Some(cost),
            input_tokens: Some(input_tokens),
            output_tokens: Some(output_tokens),
            model: Some("claude-3-5-sonnet".into()),
            provider: Some("anthropic".into()),
            success: Some(true),
            end_timestamp_ns: Some(5_000),
            ..Default::default()
        },
    }
}

#[test]
fn cost_attr_matches_journal_value() {
    let ev = make_llm_event(0.0045, 1000, 500);
    let span = journal_event_to_span(&ev);
    let cost = genai_attrs::get_float(&span, ANCORA_COST_USD).unwrap();
    assert!((cost - 0.0045).abs() < 1e-9);
}

#[test]
fn token_attrs_match_journal() {
    let ev = make_llm_event(0.001, 2000, 800);
    let span = journal_event_to_span(&ev);
    assert_eq!(
        genai_attrs::get_int(&span, GEN_AI_USAGE_INPUT_TOKENS),
        Some(2000)
    );
    assert_eq!(
        genai_attrs::get_int(&span, GEN_AI_USAGE_OUTPUT_TOKENS),
        Some(800)
    );
}

#[test]
fn zero_cost_attr_present() {
    let ev = make_llm_event(0.0, 0, 0);
    let span = journal_event_to_span(&ev);
    let cost = genai_attrs::get_float(&span, ANCORA_COST_USD).unwrap();
    assert!((cost - 0.0).abs() < 1e-12);
}

#[test]
fn cost_attr_absent_when_not_in_journal() {
    let ev = JournalEvent {
        kind: JournalEventKind::RunStarted,
        timestamp_ns: 0,
        trace_id: TraceId("t".into()),
        span_id: SpanId("s".into()),
        parent_id: None,
        name: "run".into(),
        metadata: JournalMetadata {
            tenant_id: Some("t1".into()),
            run_id: Some("r1".into()),
            agent_id: Some("a1".into()),
            ..Default::default()
        },
    };
    let span = journal_event_to_span(&ev);
    assert!(!span.attributes.contains_key(ANCORA_COST_USD));
}

#[test]
fn span_duration_reflects_journal_timestamps() {
    let ev = make_llm_event(0.002, 500, 200);
    let span = journal_event_to_span(&ev);
    let dur = span.duration_ns().unwrap();
    assert_eq!(dur, 4_000); // end=5000, start=1000
}
