/// Tests: OTLP export to mock collector.
use crate::export::{
    ExportBatch, ExportResult, MockCollector, NoopExporter, OtlpConfig, OtlpExporter, Resource,
    SpanExporter,
};
use crate::genai_attrs;
use crate::span::{Span, SpanStatus};
use crate::trace::Trace;

fn make_trace_with_spans(n: usize) -> Trace {
    let root = Span::root("root", 0);
    let root_id = root.span_id.clone();
    let tid = root.trace_id.clone();
    let mut trace = Trace::new(tid.clone(), root);
    for i in 0..n {
        let mut child = Span::child(
            &format!("child-{}", i),
            root_id.clone(),
            tid.clone(),
            (i as u64 + 1) * 1000,
        );
        child.finish((i as u64 + 2) * 1000, SpanStatus::Ok);
        trace.add_span(child).unwrap();
    }
    trace
}

#[test]
fn mock_collector_receives_all_spans() {
    let trace = make_trace_with_spans(4);
    let resource = Resource::new("ancora", "0.1.0");
    let batch = ExportBatch::from_trace(&trace, resource);
    let collector = MockCollector::new();
    let result = collector.export(&batch);
    assert_eq!(result, ExportResult::Success { span_count: 5 });
    assert_eq!(collector.collected().len(), 5);
}

#[test]
fn mock_collector_clear_resets() {
    let trace = make_trace_with_spans(2);
    let resource = Resource::new("ancora", "0.1.0");
    let batch = ExportBatch::from_trace(&trace, resource);
    let collector = MockCollector::new();
    collector.export(&batch);
    collector.clear();
    assert_eq!(collector.collected().len(), 0);
}

#[test]
fn exported_span_has_correct_trace_id() {
    let trace = make_trace_with_spans(0);
    let resource = Resource::new("ancora", "0.1.0");
    let batch = ExportBatch::from_trace(&trace, resource);
    let collector = MockCollector::new();
    collector.export(&batch);
    let spans = collector.collected();
    assert_eq!(spans.len(), 1);
    assert_eq!(
        spans[0].trace_id,
        trace.get_span(&trace.root_id).unwrap().trace_id.0
    );
}

#[test]
fn exported_span_attributes_include_genai_keys() {
    let mut root = Span::root("llm-call", 0);
    genai_attrs::set_request_attrs(
        &mut root,
        "anthropic",
        "claude-3-5-sonnet",
        Some(4096),
        None,
    );
    genai_attrs::set_cost_attr(&mut root, 0.0012);
    let tid = root.trace_id.clone();
    let trace = Trace::new(tid, root);
    let resource = Resource::new("ancora", "0.1.0");
    let batch = ExportBatch::from_trace(&trace, resource);
    let collector = MockCollector::new();
    collector.export(&batch);
    let spans = collector.collected();
    let attrs: std::collections::HashMap<_, _> = spans[0].attributes.iter().cloned().collect();
    assert_eq!(
        attrs.get("gen_ai.system").map(|s| s.as_str()),
        Some("anthropic")
    );
}

#[test]
fn otlp_exporter_wraps_mock() {
    let config = OtlpConfig::new("http://collector:4318/v1/traces")
        .with_header("Authorization", "Bearer token123");
    // Build OTLP exporter wrapping the mock.
    let exporter = OtlpExporter::new(config, MockCollector::new());
    let trace = make_trace_with_spans(1);
    let resource = Resource::new("ancora", "0.1.0");
    let batch = ExportBatch::from_trace(&trace, resource);
    let result = exporter.export(&batch);
    assert_eq!(result, ExportResult::Success { span_count: 2 });
}

#[test]
fn noop_exporter_always_succeeds() {
    let trace = make_trace_with_spans(10);
    let resource = Resource::new("ancora", "0.1.0");
    let batch = ExportBatch::from_trace(&trace, resource);
    let result = NoopExporter.export(&batch);
    assert_eq!(result, ExportResult::Success { span_count: 11 });
}

#[test]
fn resource_attributes_in_batch() {
    let resource = Resource::new("ancora", "0.1.0")
        .with_attr("deployment.environment", "production")
        .with_attr("cloud.provider", "aws");
    let root = Span::root("r", 0);
    let tid = root.trace_id.clone();
    let trace = Trace::new(tid, root);
    let batch = ExportBatch::from_trace(&trace, resource);
    assert_eq!(batch.resource.service_name, "ancora");
    assert_eq!(batch.resource.attributes.len(), 2);
}
