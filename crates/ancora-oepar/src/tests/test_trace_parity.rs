use crate::trace_parity::{compare_traces, reference_trace, Language};

#[test]
fn test_reference_traces_have_two_spans() {
    let trace = reference_trace(Language::Rust);
    assert_eq!(trace.span_count(), 2);
}

#[test]
fn test_rust_python_trace_parity() {
    let rust_trace = reference_trace(Language::Rust);
    let python_trace = reference_trace(Language::Python);
    let result = compare_traces(&rust_trace, &python_trace);
    assert!(
        result.is_equal,
        "rust vs python trace differs: {:?}",
        result.differences
    );
}

#[test]
fn test_all_six_languages_parity() {
    let languages = Language::all();
    let traces: Vec<_> = languages
        .iter()
        .map(|l| reference_trace(l.clone()))
        .collect();
    let reference = &traces[0];
    for other in traces.iter().skip(1) {
        let result = compare_traces(reference, other);
        assert!(
            result.is_equal,
            "{:?} vs {:?} trace parity failed: {:?}",
            reference.language.as_str(),
            other.language.as_str(),
            result.differences
        );
    }
}

#[test]
fn test_trace_span_names_match() {
    let rust_trace = reference_trace(Language::Rust);
    let go_trace = reference_trace(Language::Go);
    for (a, b) in rust_trace.spans.iter().zip(go_trace.spans.iter()) {
        assert_eq!(a.name, b.name, "span names must match across languages");
    }
}

#[test]
fn test_span_attributes_present() {
    let trace = reference_trace(Language::TypeScript);
    for span in &trace.spans {
        assert!(
            span.attributes.contains_key("gen_ai.system"),
            "gen_ai.system attribute missing on span {:?}",
            span.name
        );
    }
}
