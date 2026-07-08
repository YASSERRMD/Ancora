/// Tests: trace id propagates correctly across agent-to-agent calls.
use crate::propagation::{HeaderCarrier, TraceContext};
use crate::span::{Span, SpanId, TraceId};

#[test]
fn traceparent_carries_trace_id_across_boundary() {
    let root = Span::root("parent-agent", 0);
    let ctx = TraceContext::sampled(root.trace_id.clone(), root.span_id.clone());

    let mut carrier = HeaderCarrier::new();
    carrier.inject(&ctx);

    // Simulate receiving on the child agent side.
    let received = carrier.extract().unwrap();
    assert_eq!(received.trace_id, root.trace_id);
}

#[test]
fn child_agent_uses_parent_span_as_parent_id() {
    let parent_span_id = SpanId("parent-span-123".into());
    let trace_id = TraceId("trace-abc".into());
    let ctx = TraceContext::sampled(trace_id.clone(), parent_span_id.clone());

    let mut carrier = HeaderCarrier::new();
    carrier.inject(&ctx);

    let received = carrier.extract().unwrap();
    let child = Span::child(
        "child-agent-run",
        received.parent_span_id.clone(),
        received.trace_id.clone(),
        1000,
    );
    assert_eq!(child.parent_id.as_ref(), Some(&parent_span_id));
    assert_eq!(child.trace_id, trace_id);
}

#[test]
fn sampled_flag_preserved() {
    let ctx = TraceContext::sampled(TraceId("t".into()), SpanId("s".into()));
    let mut c = HeaderCarrier::new();
    c.inject(&ctx);
    let recv = c.extract().unwrap();
    assert!(recv.is_sampled());
}

#[test]
fn unsampled_flag_preserved() {
    let ctx = TraceContext::unsampled(TraceId("t".into()), SpanId("s".into()));
    let mut c = HeaderCarrier::new();
    c.inject(&ctx);
    let recv = c.extract().unwrap();
    assert!(!recv.is_sampled());
}

#[test]
fn invalid_traceparent_returns_none() {
    let result = TraceContext::from_traceparent("garbage-value");
    assert!(result.is_none());
}

#[test]
fn multiple_hops_preserve_original_trace_id() {
    let original_trace = TraceId("original-trace-id".into());
    let ctx = TraceContext::sampled(original_trace.clone(), SpanId("s1".into()));

    // Hop 1
    let mut c1 = HeaderCarrier::new();
    c1.inject(&ctx);
    let recv1 = c1.extract().unwrap();

    // Hop 2 - child re-injects its own span but keeps the same trace id
    let ctx2 = TraceContext::sampled(recv1.trace_id.clone(), SpanId("s2".into()));
    let mut c2 = HeaderCarrier::new();
    c2.inject(&ctx2);
    let recv2 = c2.extract().unwrap();

    assert_eq!(recv2.trace_id, original_trace);
}

#[test]
fn trace_spans_from_two_agents_share_trace_id() {
    let root = Span::root("agent-1", 0);
    let root_id = root.span_id.clone();
    let tid = root.trace_id.clone();

    let ctx = TraceContext::sampled(tid.clone(), root_id.clone());
    let mut carrier = HeaderCarrier::new();
    carrier.inject(&ctx);
    let recv = carrier.extract().unwrap();

    let child = Span::child("agent-2-run", recv.parent_span_id, recv.trace_id, 1000);
    assert_eq!(child.trace_id, root.trace_id);
}
