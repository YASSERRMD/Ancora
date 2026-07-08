use crate::context::CostRecord;
use crate::rs_helpers::{RsCostAccessor, RsTraceAccessor};

#[test]
fn rs_trace_accessor_returns_spans() {
    let mut acc = RsTraceAccessor::new("rs-t-001");
    acc.record_span("s1", "agent.run", 0, 5000);
    acc.record_child_span("s2", "s1", "tool.call", 100, 3000);
    assert_eq!(acc.span_count(), 2);
    let spans = acc.spans();
    assert_eq!(spans[0].span_id, "s1");
    assert_eq!(spans[1].name, "tool.call");
}

#[test]
fn rs_trace_root_duration() {
    let mut acc = RsTraceAccessor::new("rs-t-002");
    acc.record_span("s1", "pipeline", 0, 999);
    assert_eq!(acc.root_duration_ns(), Some(999));
}

#[test]
fn rs_cost_iter_models() {
    let mut acc = RsCostAccessor::new();
    acc.record(CostRecord::new("rs-t-001", 100, 50, "claude-3-haiku"));
    acc.record(CostRecord::new("rs-t-001", 200, 100, "claude-3-sonnet"));
    let models: Vec<&str> = acc.iter_models().collect();
    assert_eq!(models.len(), 2);
    assert!(models.contains(&"claude-3-haiku"));
}
