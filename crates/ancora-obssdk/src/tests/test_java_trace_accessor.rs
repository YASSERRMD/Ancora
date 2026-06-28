use crate::java_helpers::{JavaCostAccessor, JavaTraceAccessor};
use crate::context::CostRecord;

#[test]
fn java_trace_accessor_returns_spans() {
    let mut acc = JavaTraceAccessor::new("java-t-001");
    acc.start_span("s1", "Servlet.doGet", 0, 3000);
    acc.start_child_span("s2", "s1", "JDBC.executeQuery", 100, 2000);
    assert_eq!(acc.span_count(), 2);
    let names = acc.span_names();
    assert!(names.contains(&"Servlet.doGet"));
    assert!(names.contains(&"JDBC.executeQuery"));
}

#[test]
fn java_trace_parent_id() {
    let mut acc = JavaTraceAccessor::new("java-t-002");
    acc.start_span("s1", "root", 0, 1000);
    acc.start_child_span("s2", "s1", "child", 100, 900);
    assert_eq!(acc.spans()[1].parent_id.as_deref(), Some("s1"));
}

#[test]
fn java_cost_to_string_format() {
    let mut acc = JavaCostAccessor::new();
    acc.record(CostRecord::new("java-t-001", 700, 350, "claude-3-opus"));
    let s = acc.to_string();
    assert!(s.contains("1050"));
    assert!(s.contains("JavaCostAccessor"));
}
