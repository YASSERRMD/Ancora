use crate::polyglot::{
    reference_polyglot_trace, shared_eval_case_ids, stitch_polyglot_trace, A2AContext,
    SHARED_EVAL_DATASET_ID,
};
use crate::trace_parity::{Language, Span};

#[test]
fn test_reference_polyglot_trace_has_six_spans() {
    let trace = reference_polyglot_trace();
    assert_eq!(trace.span_count(), 6, "expected one span per language");
}

#[test]
fn test_polyglot_trace_has_all_six_languages() {
    let trace = reference_polyglot_trace();
    let langs = trace.contributing_languages();
    assert_eq!(langs.len(), 6, "all six languages should contribute spans");
}

#[test]
fn test_polyglot_parent_links_are_valid() {
    let trace = reference_polyglot_trace();
    let errors = trace.validate_parent_links();
    assert!(errors.is_empty(), "parent link errors: {:?}", errors);
}

#[test]
fn test_a2a_context_propagation() {
    let ctx = A2AContext::new("trace-a2a-001", "span-root")
        .with_baggage("session_id", "sess-42")
        .with_baggage("user_id", "usr-7");
    assert_eq!(ctx.trace_id, "trace-a2a-001");
    assert_eq!(ctx.baggage.len(), 2);
}

#[test]
fn test_shared_eval_dataset_id_is_stable() {
    assert_eq!(SHARED_EVAL_DATASET_ID, "ancora-oepar-v1");
}

#[test]
fn test_shared_eval_case_ids_count() {
    let ids = shared_eval_case_ids();
    assert_eq!(ids.len(), 3, "expected 3 shared case ids");
}

#[test]
fn test_stitch_from_manual_contributions() {
    let tid = "trace-manual-001";
    let rust_span = Span::new(tid, "span-r1", "rust.root", Language::Rust);
    let py_span =
        Span::new(tid, "span-p1", "python.child", Language::Python).with_parent("span-r1");
    let trace = stitch_polyglot_trace(
        tid,
        &[
            (Language::Rust, vec![rust_span]),
            (Language::Python, vec![py_span]),
        ],
    );
    assert_eq!(trace.span_count(), 2);
    let errors = trace.validate_parent_links();
    assert!(
        errors.is_empty(),
        "unexpected parent link errors: {:?}",
        errors
    );
}

#[test]
fn test_find_span_by_id() {
    let trace = reference_polyglot_trace();
    let span = trace.find_span("span-rust-1");
    assert!(span.is_some(), "span-rust-1 should be found");
    assert_eq!(span.unwrap().name, "rust.orchestrate");
}
