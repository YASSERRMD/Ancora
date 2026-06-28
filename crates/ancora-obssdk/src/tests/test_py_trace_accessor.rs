use crate::py_helpers::{PyCostAccessor, PyTraceAccessor};
use crate::context::CostRecord;

#[test]
fn py_trace_accessor_returns_spans() {
    let mut acc = PyTraceAccessor::new("py-t-001");
    acc.record_span("s1", "llm.invoke", 0, 2000);
    acc.record_child_span("s2", "s1", "tokenize", 50, 300);
    assert_eq!(acc.span_count(), 2);
    let spans = acc.spans();
    assert_eq!(spans[0].name, "llm.invoke");
    assert_eq!(spans[1].parent_id.as_deref(), Some("s1"));
}

#[test]
fn py_trace_accessor_to_dict() {
    let mut acc = PyTraceAccessor::new("py-t-002");
    acc.record_span("s1", "agent.step", 0, 100);
    let dict = acc.to_dict();
    assert_eq!(dict.len(), 1);
    assert_eq!(dict[0].0, "s1");
    assert_eq!(dict[0].1, "agent.step");
}

#[test]
fn py_cost_accessor_summary() {
    let mut acc = PyCostAccessor::new();
    acc.record(CostRecord::new("py-t-001", 400, 200, "claude-3"));
    let summary = acc.summarize();
    assert!(summary.contains("600"));
    assert!(summary.contains("records=1"));
}
