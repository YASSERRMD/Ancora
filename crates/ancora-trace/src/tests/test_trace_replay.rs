/// Tests: trace is reproducible when replayed from the same journal.

use crate::journal::{JournalEvent, JournalEventKind, JournalMetadata, spans_from_journal};
use crate::span::{SpanId, TraceId};
use crate::trace::build_trace_from_spans;

fn sample_journal() -> Vec<JournalEvent> {
    vec![
        JournalEvent {
            kind: JournalEventKind::RunStarted,
            timestamp_ns: 1_000,
            trace_id: TraceId("trace-replay-1".into()),
            span_id: SpanId("span-root".into()),
            parent_id: None,
            name: "agent-run".into(),
            metadata: JournalMetadata {
                tenant_id: Some("tenant-1".into()),
                run_id: Some("run-replay".into()),
                agent_id: Some("agent-1".into()),
                ..Default::default()
            },
        },
        JournalEvent {
            kind: JournalEventKind::LlmCallStarted,
            timestamp_ns: 2_000,
            trace_id: TraceId("trace-replay-1".into()),
            span_id: SpanId("span-llm-1".into()),
            parent_id: Some(SpanId("span-root".into())),
            name: "llm-call".into(),
            metadata: JournalMetadata {
                provider: Some("anthropic".into()),
                model: Some("claude-3-opus".into()),
                input_tokens: Some(500),
                output_tokens: Some(200),
                cost_usd: Some(0.001),
                success: Some(true),
                end_timestamp_ns: Some(4_000),
                tenant_id: Some("tenant-1".into()),
                run_id: Some("run-replay".into()),
                agent_id: Some("agent-1".into()),
                ..Default::default()
            },
        },
        JournalEvent {
            kind: JournalEventKind::ToolCallStarted,
            timestamp_ns: 5_000,
            trace_id: TraceId("trace-replay-1".into()),
            span_id: SpanId("span-tool-1".into()),
            parent_id: Some(SpanId("span-root".into())),
            name: "tool-call".into(),
            metadata: JournalMetadata {
                tool_name: Some("search".into()),
                success: Some(true),
                end_timestamp_ns: Some(6_000),
                tenant_id: Some("tenant-1".into()),
                run_id: Some("run-replay".into()),
                agent_id: Some("agent-1".into()),
                ..Default::default()
            },
        },
    ]
}

#[test]
fn replay_produces_same_span_count() {
    let journal = sample_journal();
    let spans1 = spans_from_journal(&journal);
    let spans2 = spans_from_journal(&journal);
    assert_eq!(spans1.len(), spans2.len());
}

#[test]
fn replay_produces_same_span_names() {
    let journal = sample_journal();
    let spans1 = spans_from_journal(&journal);
    let spans2 = spans_from_journal(&journal);

    let names1: Vec<&str> = spans1.iter().map(|s| s.name.as_str()).collect();
    let names2: Vec<&str> = spans2.iter().map(|s| s.name.as_str()).collect();
    assert_eq!(names1, names2);
}

#[test]
fn replay_produces_same_trace_ids() {
    let journal = sample_journal();
    let spans1 = spans_from_journal(&journal);
    let spans2 = spans_from_journal(&journal);

    let tids1: Vec<&str> = spans1.iter().map(|s| s.trace_id.0.as_str()).collect();
    let tids2: Vec<&str> = spans2.iter().map(|s| s.trace_id.0.as_str()).collect();
    assert_eq!(tids1, tids2);
}

#[test]
fn replay_trace_tree_has_correct_structure() {
    let journal = sample_journal();
    let spans = spans_from_journal(&journal);
    let trace = build_trace_from_spans(spans).unwrap();

    assert_eq!(trace.span_count(), 3);
    let children = trace.children_of(&trace.root_id);
    assert_eq!(children.len(), 2);
}

#[test]
fn replay_span_attributes_identical() {
    let journal = sample_journal();
    let spans1 = spans_from_journal(&journal);
    let spans2 = spans_from_journal(&journal);

    for (s1, s2) in spans1.iter().zip(spans2.iter()) {
        assert_eq!(s1.attributes.len(), s2.attributes.len());
        for (k, v) in &s1.attributes {
            assert_eq!(s2.attributes.get(k), Some(v));
        }
    }
}
