use crate::context::CostRecord;
use crate::ts_helpers::{TsCostAccessor, TsTraceAccessor};

#[test]
fn ts_trace_accessor_returns_spans() {
    let mut acc = TsTraceAccessor::new("ts-t-001");
    acc.record_span("s1", "api.fetch", 0, 1500);
    acc.record_child_span("s2", "s1", "parse.response", 200, 800);
    assert_eq!(acc.span_count(), 2);
    let spans = acc.spans();
    assert_eq!(spans[0].name, "api.fetch");
    assert_eq!(spans[1].name, "parse.response");
}

#[test]
fn ts_trace_accessor_json_output() {
    let mut acc = TsTraceAccessor::new("ts-t-002");
    acc.record_span("s1", "render", 0, 100);
    let json_strs = acc.to_json_strings();
    assert_eq!(json_strs.len(), 1);
    assert!(json_strs[0].contains("\"name\":\"render\""));
    assert!(json_strs[0].contains("\"spanId\":\"s1\""));
}

#[test]
fn ts_cost_accessor_total() {
    let mut acc = TsCostAccessor::new();
    acc.record(CostRecord::new("ts-t-001", 150, 75, "claude-3"));
    assert_eq!(acc.total_tokens(), 225);
}
