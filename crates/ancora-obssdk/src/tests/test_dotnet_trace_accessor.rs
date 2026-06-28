use crate::dotnet_helpers::{DotnetCostAccessor, DotnetTraceAccessor};
use crate::context::CostRecord;

#[test]
fn dotnet_trace_accessor_returns_spans() {
    let mut acc = DotnetTraceAccessor::new("dotnet-t-001");
    acc.start_activity("s1", "MVC.Action", 0, 2000);
    acc.start_child_activity("s2", "s1", "EF.Query", 100, 900);
    assert_eq!(acc.span_count(), 2);
    let spans = acc.spans();
    assert_eq!(spans[0].name, "MVC.Action");
    assert_eq!(spans[1].parent_id.as_deref(), Some("s1"));
}

#[test]
fn dotnet_traceparent_contains_trace_id() {
    let mut acc = DotnetTraceAccessor::new("dotnet-trace-abc");
    acc.start_activity("s1", "op", 0, 100);
    let tp = acc.traceparent();
    assert!(tp.contains("dotnet-trace-abc"));
    assert!(tp.starts_with("00-"));
}

#[test]
fn dotnet_cost_accessor() {
    let mut acc = DotnetCostAccessor::new();
    acc.record(CostRecord::new("dotnet-t-001", 600, 300, "claude-3"));
    assert_eq!(acc.total_tokens(), 900);
    assert_eq!(acc.records().len(), 1);
}
