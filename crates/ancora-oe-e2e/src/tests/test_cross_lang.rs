/// Cross-language trace stitching tests.
/// Simulates traces that originate from multiple language runtimes (e.g. Python + Rust)
/// and verifies they can be stitched by a shared trace_id and span parent references.
use crate::trace_e2e::{Span, Trace};

/// Simulates spans emitted by a Python orchestrator.
fn python_spans(trace_id: &str) -> Vec<Span> {
    vec![Span::new("py-root", "python.orchestrate", 0, 2_000_000)
        .with_attribute("lang", "python")
        .with_attribute("trace.id", trace_id)]
}

/// Simulates spans emitted by a Rust agent.
fn rust_spans(trace_id: &str) -> Vec<Span> {
    vec![
        Span::new("rs-agent", "rust.agent.run", 100_000, 1_800_000)
            .with_parent("py-root")
            .with_attribute("lang", "rust")
            .with_attribute("trace.id", trace_id),
        Span::new("rs-llm", "rust.llm.invoke", 200_000, 1_500_000)
            .with_parent("rs-agent")
            .with_attribute("lang", "rust")
            .with_attribute("trace.id", trace_id),
    ]
}

/// Stitches multi-language spans into a single trace.
fn stitch_trace(trace_id: &str, span_batches: &[Vec<Span>]) -> Trace {
    let mut trace = Trace::new(trace_id);
    for batch in span_batches {
        for span in batch {
            trace.add_span(span.clone());
        }
    }
    trace
}

#[test]
fn cross_language_trace_stitching_produces_complete_trace() {
    let trace_id = "cross-lang-001";
    let py = python_spans(trace_id);
    let rs = rust_spans(trace_id);

    let trace = stitch_trace(trace_id, &[py, rs]);

    assert_eq!(trace.trace_id, trace_id);
    assert_eq!(
        trace.spans.len(),
        3,
        "must have all 3 spans (1 python + 2 rust)"
    );

    let root = trace.root_span().expect("must have a root span");
    assert_eq!(root.span_id, "py-root");
    assert_eq!(root.attributes["lang"], "python");
}

#[test]
fn child_spans_are_correctly_linked_across_languages() {
    let trace_id = "cross-lang-002";
    let trace = stitch_trace(trace_id, &[python_spans(trace_id), rust_spans(trace_id)]);

    let root_children = trace.child_spans("py-root");
    assert_eq!(root_children.len(), 1);
    assert_eq!(root_children[0].span_id, "rs-agent");

    let agent_children = trace.child_spans("rs-agent");
    assert_eq!(agent_children.len(), 1);
    assert_eq!(agent_children[0].span_id, "rs-llm");
}

#[test]
fn all_spans_share_the_same_trace_id_attribute() {
    let trace_id = "cross-lang-003";
    let trace = stitch_trace(trace_id, &[python_spans(trace_id), rust_spans(trace_id)]);

    for span in &trace.spans {
        assert_eq!(
            span.attributes.get("trace.id").map(|s| s.as_str()),
            Some(trace_id),
            "span '{}' must carry the correct trace.id attribute",
            span.name
        );
    }
}
