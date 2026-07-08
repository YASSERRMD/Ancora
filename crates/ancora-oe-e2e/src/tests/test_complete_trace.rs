use crate::trace_e2e::{build_run_trace, Span, Trace};

#[test]
fn run_produces_a_complete_trace() {
    let trace = build_run_trace("trace-001");

    assert_eq!(trace.trace_id, "trace-001");
    assert!(trace.is_complete(), "trace must have at least one span");

    let root = trace.root_span().expect("trace must have a root span");
    assert_eq!(root.name, "agent.run");
    assert!(root.parent_id.is_none());

    let children = trace.child_spans(&root.span_id);
    assert_eq!(children.len(), 2, "root span must have two children");

    let total_duration = trace.total_duration_ns();
    assert!(total_duration > 0, "total duration must be positive");
}

#[test]
fn trace_spans_have_valid_time_ranges() {
    let trace = build_run_trace("trace-002");
    for span in &trace.spans {
        assert!(
            span.end_ns >= span.start_ns,
            "span '{}' end must be >= start",
            span.name
        );
    }
}

#[test]
fn trace_attributes_are_accessible() {
    let trace = build_run_trace("trace-003");
    let root = trace.root_span().unwrap();
    assert_eq!(
        root.attributes.get("agent.name").map(|s| s.as_str()),
        Some("test-agent")
    );
}

#[test]
fn empty_trace_is_not_complete() {
    let trace = Trace::new("empty");
    assert!(!trace.is_complete());
    assert!(trace.root_span().is_none());
}
