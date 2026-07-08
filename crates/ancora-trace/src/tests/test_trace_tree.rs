/// Tests: trace tree mirrors run structure.
use crate::span::{Span, SpanStatus};
use crate::trace::{build_trace_from_spans, Trace};

#[test]
fn trace_tree_root_has_no_parent() {
    let root = Span::root("agent-run", 1_000);
    let tid = root.trace_id.clone();
    let trace = Trace::new(tid, root.clone());
    assert_eq!(trace.root_id, root.span_id);
    let r = trace.get_span(&trace.root_id).unwrap();
    assert!(r.parent_id.is_none());
}

#[test]
fn trace_tree_children_linked_correctly() {
    let root = Span::root("agent-run", 1_000);
    let root_id = root.span_id.clone();
    let tid = root.trace_id.clone();
    let mut trace = Trace::new(tid.clone(), root);

    let child1 = Span::child("tool-call", root_id.clone(), tid.clone(), 2_000);
    let child2 = Span::child("llm-call", root_id.clone(), tid.clone(), 3_000);
    trace.add_span(child1).unwrap();
    trace.add_span(child2).unwrap();

    let children = trace.children_of(&root_id);
    assert_eq!(children.len(), 2);
}

#[test]
fn trace_tree_span_count() {
    let root = Span::root("run", 0);
    let tid = root.trace_id.clone();
    let rid = root.span_id.clone();
    let mut trace = Trace::new(tid.clone(), root);

    for i in 0..5u64 {
        let child = Span::child(&format!("child-{}", i), rid.clone(), tid.clone(), i * 100);
        trace.add_span(child).unwrap();
    }

    assert_eq!(trace.span_count(), 6);
}

#[test]
fn build_trace_from_flat_list() {
    let root = Span::root("run", 0);
    let root_id = root.span_id.clone();
    let tid = root.trace_id.clone();
    let child = Span::child("child", root_id, tid, 500);

    let spans = vec![root, child];
    let trace = build_trace_from_spans(spans).unwrap();
    assert_eq!(trace.span_count(), 2);
}

#[test]
fn all_spans_ordered_by_start_ns() {
    let root = Span::root("run", 1000);
    let rid = root.span_id.clone();
    let tid = root.trace_id.clone();
    let mut trace = Trace::new(tid.clone(), root);

    let c1 = Span::child("b", rid.clone(), tid.clone(), 3000);
    let c2 = Span::child("a", rid.clone(), tid.clone(), 2000);
    trace.add_span(c1).unwrap();
    trace.add_span(c2).unwrap();

    let ordered: Vec<u64> = trace.all_spans().iter().map(|s| s.start_ns).collect();
    assert!(ordered.windows(2).all(|w| w[0] <= w[1]));
}

#[test]
fn finished_span_has_end_time() {
    let mut root = Span::root("run", 0);
    root.finish(10_000, SpanStatus::Ok);
    let tid = root.trace_id.clone();
    let trace = Trace::new(tid, root.clone());
    let span = trace.get_span(&root.span_id).unwrap();
    assert!(span.end_ns.is_some());
}
