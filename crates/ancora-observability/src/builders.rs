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

/// Build a node span that also records the finish reason from the model.
pub fn node_span_with_finish_reason(
    run_id: &str,
    node_id: &str,
    node_kind: &str,
    model: &str,
    tokens_in: u64,
    tokens_out: u64,
    cost_usd: f64,
    finish_reason: &str,
) -> Span {
    node_span(
        run_id, node_id, node_kind, model, tokens_in, tokens_out, cost_usd,
    )
    .set(GEN_AI_RESPONSE_FINISH_REASON, finish_reason)
}

/// Build a span for a completed graph run with total cost.
pub fn graph_span(run_id: &str, graph_id: &str, total_cost_usd: f64) -> Span {
    Span::new("ancora.graph")
        .set(GEN_AI_OPERATION_NAME, "graph")
        .set(ANCORA_RUN_ID, run_id)
        .set(ANCORA_GRAPH_ID, graph_id)
        .set(ANCORA_TOTAL_COST_USD, total_cost_usd)
}

/// Build a span from a journal event name.
pub fn journal_event_span(run_id: &str, event_name: &str) -> Span {
    Span::new(event_name)
        .set(ANCORA_RUN_ID, run_id)
        .set("event.name", event_name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::span::SpanValue;

    #[test]
    fn run_span_carries_genai_operation_and_system() {
        let span = run_span("run-1", "chat", "openai");
        assert_eq!(
            span.attributes.get(GEN_AI_OPERATION_NAME),
            Some(&SpanValue::String("chat".into()))
        );
        assert_eq!(
            span.attributes.get(GEN_AI_SYSTEM),
            Some(&SpanValue::String("openai".into()))
        );
        assert_eq!(
            span.attributes.get(ANCORA_RUN_ID),
            Some(&SpanValue::String("run-1".into()))
        );
    }

    #[test]
    fn node_span_carries_model_and_token_usage() {
        let span = node_span("run-2", "n1", "agent", "gpt-4o", 100, 50, 0.002);
        assert_eq!(
            span.attributes.get(GEN_AI_REQUEST_MODEL),
            Some(&SpanValue::String("gpt-4o".into()))
        );
        assert_eq!(
            span.attributes.get(GEN_AI_USAGE_INPUT_TOKENS),
            Some(&SpanValue::Int(100))
        );
        assert_eq!(
            span.attributes.get(GEN_AI_USAGE_OUTPUT_TOKENS),
            Some(&SpanValue::Int(50))
        );
    }

    #[test]
    fn node_span_carries_cost_and_node_ids() {
        let span = node_span("run-3", "n2", "tool", "gpt-4o-mini", 0, 0, 0.0);
        assert_eq!(
            span.attributes.get(ANCORA_NODE_ID),
            Some(&SpanValue::String("n2".into()))
        );
        assert_eq!(
            span.attributes.get(ANCORA_NODE_KIND),
            Some(&SpanValue::String("tool".into()))
        );
        assert_eq!(
            span.attributes.get(ANCORA_COST_USD),
            Some(&SpanValue::Float(0.0))
        );
    }

    #[test]
    fn journal_event_span_carries_event_name() {
        let span = journal_event_span("run-4", "node.started");
        assert_eq!(
            span.attributes.get("event.name"),
            Some(&SpanValue::String("node.started".into()))
        );
        assert_eq!(
            span.attributes.get(ANCORA_RUN_ID),
            Some(&SpanValue::String("run-4".into()))
        );
    }

    #[test]
    fn graph_span_carries_graph_id_and_total_cost() {
        let span = graph_span("run-5", "graph-abc", 0.05);
        assert_eq!(
            span.attributes.get(ANCORA_GRAPH_ID),
            Some(&SpanValue::String("graph-abc".into()))
        );
        assert_eq!(
            span.attributes.get(ANCORA_TOTAL_COST_USD),
            Some(&SpanValue::Float(0.05))
        );
        assert_eq!(
            span.attributes.get(ANCORA_RUN_ID),
            Some(&SpanValue::String("run-5".into()))
        );
    }

    #[test]
    fn node_span_with_finish_reason_carries_finish_reason() {
        let span =
            node_span_with_finish_reason("r1", "n1", "agent", "gpt-4o", 10, 5, 0.001, "stop");
        assert_eq!(
            span.attributes.get(GEN_AI_RESPONSE_FINISH_REASON),
            Some(&SpanValue::String("stop".into()))
        );
    }
}
