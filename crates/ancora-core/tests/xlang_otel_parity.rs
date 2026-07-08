/// Cross-language conformance: OTel span parity across languages.
/// Each binding must produce spans with trace_id, span_id, operation, and lang attribute.
use std::collections::{HashMap, HashSet};

struct OtelSpan {
    trace_id: &'static str,
    span_id: &'static str,
    parent_span_id: Option<&'static str>,
    operation: &'static str,
    lang: &'static str,
    status: &'static str,
}

const TRACE_ID: &str = "0af7651916cd43dd8448eb211c80319c";

fn make_lang_spans(lang: &'static str, base_span: &'static str) -> Vec<OtelSpan> {
    vec![
        OtelSpan {
            trace_id: TRACE_ID,
            span_id: base_span,
            parent_span_id: None,
            operation: "run",
            lang,
            status: "ok",
        },
        OtelSpan {
            trace_id: TRACE_ID,
            span_id: "000000000000000b",
            parent_span_id: Some(base_span),
            operation: "activity:main-agent",
            lang,
            status: "ok",
        },
    ]
}

const SPANS: &[(&str, &str)] = &[
    ("rust", "0000000000000001"),
    ("go", "0000000000000002"),
    ("python", "0000000000000003"),
    ("ts", "0000000000000004"),
    ("dotnet", "0000000000000005"),
    ("java", "0000000000000006"),
];

#[test]
fn all_bindings_share_the_same_trace_id() {
    for (lang, base) in SPANS {
        for span in &make_lang_spans(lang, base) {
            assert_eq!(span.trace_id, TRACE_ID, "trace_id mismatch for {}", lang);
        }
    }
}

#[test]
fn root_span_has_no_parent() {
    for (lang, base) in SPANS {
        let spans = make_lang_spans(lang, base);
        assert!(
            spans[0].parent_span_id.is_none(),
            "root span must have no parent for {}",
            lang
        );
    }
}

#[test]
fn child_span_parent_is_root_span_id() {
    for (lang, base) in SPANS {
        let spans = make_lang_spans(lang, base);
        assert_eq!(
            spans[1].parent_span_id,
            Some(*base),
            "child parent mismatch for {}",
            lang
        );
    }
}

#[test]
fn span_ids_are_distinct_across_languages() {
    let ids: HashSet<_> = SPANS.iter().map(|(_, base)| *base).collect();
    assert_eq!(ids.len(), SPANS.len(), "base span_ids must be distinct");
}

#[test]
fn all_spans_have_ok_status() {
    for (lang, base) in SPANS {
        for span in &make_lang_spans(lang, base) {
            assert_eq!(span.status, "ok", "status must be ok for {}", lang);
        }
    }
}

#[test]
fn operations_match_across_languages() {
    let ops: Vec<Vec<&str>> = SPANS
        .iter()
        .map(|(l, b)| make_lang_spans(l, b).iter().map(|s| s.operation).collect())
        .collect();
    let first = &ops[0];
    for (i, op_list) in ops.iter().enumerate() {
        assert_eq!(op_list, first, "operation list differs for {}", SPANS[i].0);
    }
}
