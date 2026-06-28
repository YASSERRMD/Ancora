use crate::go_helpers::{GoCostAccessor, GoTraceAccessor};
use crate::context::CostRecord;

#[test]
fn go_trace_accessor_returns_spans() {
    let mut acc = GoTraceAccessor::new("go-t-001");
    acc.record_span("s1", "http.handler", 0, 1000);
    acc.record_child_span("s2", "s1", "db.query", 100, 800);
    assert_eq!(acc.span_count(), 2);
    let spans = acc.spans();
    assert_eq!(spans[0].name, "http.handler");
    assert_eq!(spans[1].name, "db.query");
    assert_eq!(spans[1].parent_id.as_deref(), Some("s1"));
}

#[test]
fn go_trace_id_is_preserved() {
    let acc = GoTraceAccessor::new("go-id-xyz");
    assert_eq!(acc.trace_id(), "go-id-xyz");
}

#[test]
fn go_cost_accessor_multiple_records() {
    let mut acc = GoCostAccessor::new();
    acc.record(CostRecord::new("go-t-001", 100, 50, "claude-3-haiku"));
    acc.record(CostRecord::new("go-t-001", 200, 100, "claude-3-sonnet"));
    assert_eq!(acc.total_tokens(), 450);
    assert_eq!(acc.records().len(), 2);
}
