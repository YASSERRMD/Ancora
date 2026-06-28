/// Tests that telemetry exported after redaction contains zero sensitive data.

use crate::privacy_e2e::{assert_no_sensitive_data, default_redactor};
use crate::trace_e2e::{build_run_trace, MockCollector, TraceExporter};

/// Apply redaction to all span attributes in the trace before export.
fn redact_and_export(
    trace: &mut crate::trace_e2e::Trace,
    redactor: &crate::privacy_e2e::Redactor,
    collector: &mut MockCollector,
) -> Result<(), String> {
    for span in &mut trace.spans {
        span.attributes = redactor.redact_map(&span.attributes);
    }
    collector.export(trace)
}

#[test]
fn zero_sensitive_data_in_telemetry_after_redaction() {
    let redactor = default_redactor();
    let mut trace = build_run_trace("pii-test-001");

    // Inject PII into a span attribute to simulate a leak.
    if let Some(span) = trace.spans.first_mut() {
        span.attributes.insert("user.email".to_string(), "user@example.com".to_string());
        span.attributes.insert("api.key".to_string(), "sk-secret-12345".to_string());
    }

    let mut collector = MockCollector::new();
    redact_and_export(&mut trace, &redactor, &mut collector).expect("export must succeed");

    // Verify no PII in stored trace.
    let stored = collector.find_trace("pii-test-001").unwrap();
    for span in &stored.spans {
        for (key, value) in &span.attributes {
            let result = assert_no_sensitive_data(&redactor, value);
            assert!(
                result.is_ok(),
                "attribute '{}' still contains sensitive data: {:?}",
                key,
                result.err()
            );
        }
    }
}

#[test]
fn clean_trace_has_no_pii_to_start() {
    let redactor = default_redactor();
    let trace = build_run_trace("clean-001");

    for span in &trace.spans {
        for (_key, value) in &span.attributes {
            assert!(
                !redactor.has_sensitive_data(value),
                "default trace must not contain PII"
            );
        }
    }
}
