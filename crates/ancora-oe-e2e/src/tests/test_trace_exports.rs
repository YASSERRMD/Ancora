use crate::trace_e2e::{build_run_trace, MockCollector, Trace, TraceExporter};

#[test]
fn trace_exports_to_mock_collector() {
    let mut collector = MockCollector::new();
    let trace = build_run_trace("trace-exp-001");

    collector.export(&trace).expect("export must succeed");

    assert_eq!(collector.count(), 1);
    let stored = collector
        .find_trace("trace-exp-001")
        .expect("trace must be findable");
    assert_eq!(stored.trace_id, "trace-exp-001");
    assert_eq!(stored.spans.len(), trace.spans.len());
}

#[test]
fn exporting_multiple_traces_accumulates() {
    let mut collector = MockCollector::new();

    for i in 0..5 {
        let trace = build_run_trace(&format!("trace-multi-{}", i));
        collector.export(&trace).expect("export must succeed");
    }

    assert_eq!(collector.count(), 5);
}

#[test]
fn exporting_empty_trace_id_returns_error() {
    let mut collector = MockCollector::new();
    let trace = Trace::new("");
    let result = collector.export(&trace);
    assert!(result.is_err(), "exporting empty trace_id must fail");
}

#[test]
fn exported_trace_spans_are_preserved() {
    let mut collector = MockCollector::new();
    let trace = build_run_trace("trace-exp-002");
    let expected_span_count = trace.spans.len();

    collector.export(&trace).unwrap();

    let stored = collector.find_trace("trace-exp-002").unwrap();
    assert_eq!(stored.spans.len(), expected_span_count);
}
