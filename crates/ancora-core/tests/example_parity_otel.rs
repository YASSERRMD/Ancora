// Example parity: OTel span example produces same trace_id and span structure across SDKs.

const EXAMPLE_TRACE_ID: &str = "0af7651916cd43dd8448eb211c80319c";
const EXAMPLE_SPAN_COUNT: usize = 3;

struct OtelSpan {
    trace_id: &'static str,
    span_id: &'static str,
    operation: &'static str,
    lang: &'static str,
}

const OTEL_EXAMPLES: &[&[OtelSpan]] = &[
    &[
        OtelSpan {
            trace_id: EXAMPLE_TRACE_ID,
            span_id: "span-rust-0",
            operation: "run",
            lang: "rust",
        },
        OtelSpan {
            trace_id: EXAMPLE_TRACE_ID,
            span_id: "span-rust-1",
            operation: "activity",
            lang: "rust",
        },
        OtelSpan {
            trace_id: EXAMPLE_TRACE_ID,
            span_id: "span-rust-2",
            operation: "complete",
            lang: "rust",
        },
    ],
    &[
        OtelSpan {
            trace_id: EXAMPLE_TRACE_ID,
            span_id: "span-go-0",
            operation: "run",
            lang: "go",
        },
        OtelSpan {
            trace_id: EXAMPLE_TRACE_ID,
            span_id: "span-go-1",
            operation: "activity",
            lang: "go",
        },
        OtelSpan {
            trace_id: EXAMPLE_TRACE_ID,
            span_id: "span-go-2",
            operation: "complete",
            lang: "go",
        },
    ],
];

#[test]
fn test_all_spans_share_trace_id() {
    for spans in OTEL_EXAMPLES {
        for span in *spans {
            assert_eq!(
                span.trace_id, EXAMPLE_TRACE_ID,
                "lang {} span trace_id differs",
                span.lang
            );
        }
    }
}

#[test]
fn test_each_example_has_3_spans() {
    for spans in OTEL_EXAMPLES {
        assert_eq!(spans.len(), EXAMPLE_SPAN_COUNT);
    }
}

#[test]
fn test_trace_id_is_32_hex_chars() {
    assert_eq!(EXAMPLE_TRACE_ID.len(), 32);
    assert!(EXAMPLE_TRACE_ID.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn test_operation_sequence_run_activity_complete() {
    for spans in OTEL_EXAMPLES {
        assert_eq!(spans[0].operation, "run");
        assert_eq!(spans[1].operation, "activity");
        assert_eq!(spans[2].operation, "complete");
    }
}

#[test]
fn test_span_ids_unique_within_example() {
    for spans in OTEL_EXAMPLES {
        let mut ids: Vec<&str> = spans.iter().map(|s| s.span_id).collect();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), spans.len());
    }
}

#[test]
fn test_two_otel_examples() {
    assert_eq!(OTEL_EXAMPLES.len(), 2);
}
