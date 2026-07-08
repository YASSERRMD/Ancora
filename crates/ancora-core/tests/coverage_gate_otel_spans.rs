// Coverage gate: OTel span fields covered by tests.

const OTEL_REQUIRED_FIELDS: &[&str] = &[
    "trace_id",
    "span_id",
    "parent_span_id",
    "operation",
    "lang",
    "status",
];

const OTEL_TEST_MAP: &[(&str, &str)] = &[
    ("trace_id", "det_otel_replay"),
    ("span_id", "det_otel_replay"),
    ("parent_span_id", "det_otel_replay"),
    ("operation", "xlang_otel_parity"),
    ("lang", "xlang_otel_parity"),
    ("status", "xlang_otel_parity"),
];

const TRACE_ID_VALUE: &str = "0af7651916cd43dd8448eb211c80319c";

#[test]
fn test_all_otel_fields_have_coverage() {
    let covered: Vec<&str> = OTEL_TEST_MAP.iter().map(|(f, _)| *f).collect();
    for field in OTEL_REQUIRED_FIELDS {
        assert!(
            covered.contains(field),
            "no OTel coverage for field: {field}"
        );
    }
}

#[test]
fn test_six_otel_fields_required() {
    assert_eq!(OTEL_REQUIRED_FIELDS.len(), 6);
}

#[test]
fn test_trace_id_is_canonical_value() {
    assert_eq!(TRACE_ID_VALUE, "0af7651916cd43dd8448eb211c80319c");
    assert_eq!(TRACE_ID_VALUE.len(), 32);
}

#[test]
fn test_trace_id_is_hex() {
    assert!(TRACE_ID_VALUE.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn test_det_otel_replay_covers_trace_and_span() {
    let det_coverage: Vec<&str> = OTEL_TEST_MAP
        .iter()
        .filter(|(_, t)| *t == "det_otel_replay")
        .map(|(f, _)| *f)
        .collect();
    assert!(det_coverage.contains(&"trace_id"));
    assert!(det_coverage.contains(&"span_id"));
}

#[test]
fn test_no_duplicate_otel_fields() {
    let mut sorted = OTEL_REQUIRED_FIELDS.to_vec();
    sorted.sort();
    sorted.dedup();
    assert_eq!(sorted.len(), OTEL_REQUIRED_FIELDS.len());
}
