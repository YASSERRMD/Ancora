/// Journal-to-span bridge.
///
/// Journal events record the ground truth of what happened during an agent
/// run.  This module converts raw journal records into structured `Span`
/// instances so the trace model can be assembled from persisted data.

use crate::span::{Span, SpanId, SpanKind, SpanStatus, TraceId};
use crate::genai_attrs;

/// The kind of event recorded in the journal.
#[derive(Debug, Clone, PartialEq)]
pub enum JournalEventKind {
    RunStarted,
    RunFinished,
    ToolCallStarted,
    ToolCallFinished,
    LlmCallStarted,
    LlmCallFinished,
    AgentHandoffStarted,
    AgentHandoffFinished,
    ErrorOccurred,
}

/// A single record from the run journal.
#[derive(Debug, Clone)]
pub struct JournalEvent {
    pub kind: JournalEventKind,
    pub timestamp_ns: u64,
    pub trace_id: TraceId,
    pub span_id: SpanId,
    pub parent_id: Option<SpanId>,
    pub name: String,
    pub metadata: JournalMetadata,
}

/// Structured metadata attached to a journal event.
#[derive(Debug, Clone, Default)]
pub struct JournalMetadata {
    pub tenant_id: Option<String>,
    pub run_id: Option<String>,
    pub agent_id: Option<String>,
    pub model: Option<String>,
    pub provider: Option<String>,
    pub input_tokens: Option<i64>,
    pub output_tokens: Option<i64>,
    pub cost_usd: Option<f64>,
    pub error_kind: Option<String>,
    pub retry_count: Option<i64>,
    pub tool_name: Option<String>,
    pub prompt: Option<String>,
    pub completion: Option<String>,
    pub end_timestamp_ns: Option<u64>,
    pub success: Option<bool>,
}

/// Convert a `JournalEvent` into an open or closed `Span`.
pub fn journal_event_to_span(event: &JournalEvent) -> Span {
    let mut span = Span {
        trace_id: event.trace_id.clone(),
        span_id: event.span_id.clone(),
        parent_id: event.parent_id.clone(),
        name: event.name.clone(),
        kind: journal_kind_to_span_kind(&event.kind),
        start_ns: event.timestamp_ns,
        end_ns: event.metadata.end_timestamp_ns,
        status: derive_status(&event.metadata),
        attributes: std::collections::HashMap::new(),
        events: Vec::new(),
        links: Vec::new(),
        retry_count: event.metadata.retry_count.unwrap_or(0) as u32,
    };

    apply_metadata_to_span(&mut span, &event.metadata);
    span
}

fn journal_kind_to_span_kind(kind: &JournalEventKind) -> SpanKind {
    match kind {
        JournalEventKind::ToolCallStarted | JournalEventKind::ToolCallFinished => {
            SpanKind::Client
        }
        JournalEventKind::AgentHandoffStarted => SpanKind::Producer,
        JournalEventKind::AgentHandoffFinished => SpanKind::Consumer,
        JournalEventKind::LlmCallStarted | JournalEventKind::LlmCallFinished => {
            SpanKind::Client
        }
        _ => SpanKind::Internal,
    }
}

fn derive_status(meta: &JournalMetadata) -> SpanStatus {
    match meta.success {
        Some(true) => SpanStatus::Ok,
        Some(false) => SpanStatus::Error {
            code: 1,
            message: meta.error_kind.clone().unwrap_or_default(),
        },
        None => SpanStatus::Unset,
    }
}

fn apply_metadata_to_span(span: &mut Span, meta: &JournalMetadata) {
    if let (Some(tid), Some(rid), Some(aid)) =
        (&meta.tenant_id, &meta.run_id, &meta.agent_id)
    {
        genai_attrs::set_run_attrs(span, tid, rid, aid);
    }
    if let (Some(provider), Some(model)) = (&meta.provider, &meta.model) {
        genai_attrs::set_request_attrs(span, provider, model, None, None);
    }
    if let (Some(it), Some(ot)) = (meta.input_tokens, meta.output_tokens) {
        genai_attrs::set_response_attrs(
            span,
            meta.model.as_deref().unwrap_or(""),
            it,
            ot,
        );
    }
    if let Some(cost) = meta.cost_usd {
        genai_attrs::set_cost_attr(span, cost);
    }
    if let Some(ref ek) = meta.error_kind {
        genai_attrs::set_error_attr(
            span,
            ek,
            meta.retry_count.unwrap_or(0),
        );
    }
    if let Some(ref tn) = meta.tool_name {
        span.set_attr_str(genai_attrs::ANCORA_TOOL_NAME, tn.as_str());
    }
    if let Some(ref p) = meta.prompt {
        span.set_attr_str(genai_attrs::GEN_AI_PROMPT, p.as_str());
    }
    if let Some(ref c) = meta.completion {
        span.set_attr_str(genai_attrs::GEN_AI_COMPLETION, c.as_str());
    }
}

/// Construct all spans from an ordered list of journal events.
pub fn spans_from_journal(events: &[JournalEvent]) -> Vec<Span> {
    events.iter().map(journal_event_to_span).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::span::{SpanId, TraceId};

    fn make_run_started() -> JournalEvent {
        JournalEvent {
            kind: JournalEventKind::RunStarted,
            timestamp_ns: 1_000,
            trace_id: TraceId("trace-abc".into()),
            span_id: SpanId("span-001".into()),
            parent_id: None,
            name: "run".into(),
            metadata: JournalMetadata {
                tenant_id: Some("tenant-1".into()),
                run_id: Some("run-1".into()),
                agent_id: Some("agent-1".into()),
                ..Default::default()
            },
        }
    }

    #[test]
    fn run_started_becomes_internal_span() {
        let ev = make_run_started();
        let span = journal_event_to_span(&ev);
        assert_eq!(span.kind, SpanKind::Internal);
        assert_eq!(span.name, "run");
    }

    #[test]
    fn metadata_populates_run_attrs() {
        let ev = make_run_started();
        let span = journal_event_to_span(&ev);
        assert_eq!(
            crate::genai_attrs::get_str(&span, crate::genai_attrs::ANCORA_TENANT_ID),
            Some("tenant-1")
        );
    }

    #[test]
    fn spans_from_journal_returns_all() {
        let events = vec![make_run_started()];
        let spans = spans_from_journal(&events);
        assert_eq!(spans.len(), 1);
    }
}
