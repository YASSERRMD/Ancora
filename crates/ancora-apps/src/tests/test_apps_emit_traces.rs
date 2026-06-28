use crate::traces::{Span, Tracer};

#[test]
fn document_qa_emits_span_with_cost() {
    let mut tracer = Tracer::new();
    tracer.record(Span::new("ask", "document-qa", 12, 80, 40));
    assert_eq!(tracer.span_count(), 1);
    assert!(tracer.total_cost_usd() > 0.0);
    let spans = tracer.spans_for_app("document-qa");
    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].name, "ask");
}

#[test]
fn research_assistant_emits_span() {
    let mut tracer = Tracer::new();
    tracer.record(Span::new("research", "research-assistant", 20, 150, 60));
    assert_eq!(tracer.total_tokens(), 210);
}

#[test]
fn coding_assistant_emits_span() {
    let mut tracer = Tracer::new();
    tracer.record(Span::new("suggest", "coding-assistant", 8, 50, 30));
    assert!(tracer.spans_for_app("coding-assistant").len() == 1);
}

#[test]
fn data_analysis_emits_span() {
    let mut tracer = Tracer::new();
    tracer.record(Span::new("summarise", "data-analysis", 3, 20, 10));
    assert_eq!(tracer.span_count(), 1);
}

#[test]
fn multi_app_cost_accumulates() {
    let mut tracer = Tracer::new();
    tracer.record(Span::new("qa", "document-qa", 10, 100, 50));
    tracer.record(Span::new("research", "research-assistant", 15, 200, 80));
    tracer.record(Span::new("suggest", "coding-assistant", 5, 60, 30));
    tracer.record(Span::new("summarise", "data-analysis", 3, 40, 20));
    tracer.record(Span::new("respond", "customer-support", 7, 30, 15));
    tracer.record(Span::new("review", "compliance-review", 25, 500, 100));

    assert_eq!(tracer.span_count(), 6);
    assert!(tracer.total_cost_usd() > 0.002, "accumulated cost should be non-trivial");
}
