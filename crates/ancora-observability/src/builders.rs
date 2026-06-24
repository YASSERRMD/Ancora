use crate::attrs::*;
use crate::span::Span;

/// Build a span for a run-level event.
pub fn run_span(run_id: &str, operation: &str, system: &str) -> Span {
    Span::new(operation)
        .set(GEN_AI_OPERATION_NAME, operation)
        .set(GEN_AI_SYSTEM, system)
        .set(ANCORA_RUN_ID, run_id)
}

/// Build a span for a node-level event, including token usage and cost.
pub fn node_span(
    run_id: &str,
    node_id: &str,
    node_kind: &str,
    model: &str,
    tokens_in: u64,
    tokens_out: u64,
    cost_usd: f64,
) -> Span {
    Span::new("ancora.node")
        .set(GEN_AI_OPERATION_NAME, "chat")
        .set(GEN_AI_REQUEST_MODEL, model)
        .set(GEN_AI_USAGE_INPUT_TOKENS, tokens_in)
        .set(GEN_AI_USAGE_OUTPUT_TOKENS, tokens_out)
        .set(ANCORA_RUN_ID, run_id)
        .set(ANCORA_NODE_ID, node_id)
        .set(ANCORA_NODE_KIND, node_kind)
        .set(ANCORA_COST_USD, cost_usd)
}

/// Build a span from a journal event name.
pub fn journal_event_span(run_id: &str, event_name: &str) -> Span {
    Span::new(event_name)
        .set(ANCORA_RUN_ID, run_id)
        .set("event.name", event_name)
}
