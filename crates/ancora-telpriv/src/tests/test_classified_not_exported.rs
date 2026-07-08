use crate::classification::{classify_attr, DataClass};
use crate::span_policy::{apply_span_policy, SpanPolicy};

#[test]
fn sensitive_data_class_not_raw_in_telemetry() {
    let policy = SpanPolicy::default();
    let attrs = vec![
        ("api.key", "sk-secretvalue", DataClass::Critical),
        ("user.ssn", "123-45-6789", DataClass::Critical),
    ];
    let exported = apply_span_policy(&policy, &attrs);
    for (_, val) in &exported {
        assert_ne!(val, "sk-secretvalue");
        assert_ne!(val, "123-45-6789");
        assert_eq!(val, "[REDACTED]");
    }
}

#[test]
fn heuristic_classifier_marks_password_critical() {
    assert_eq!(classify_attr("db_password"), DataClass::Critical);
}

#[test]
fn heuristic_classifier_marks_email_sensitive() {
    assert_eq!(classify_attr("user_email"), DataClass::Sensitive);
}

#[test]
fn public_data_exported_as_is() {
    let policy = SpanPolicy::default();
    let attrs = vec![("span.name", "my_op", DataClass::Public)];
    let exported = apply_span_policy(&policy, &attrs);
    assert_eq!(exported[0].1, "my_op");
}

#[test]
fn internal_data_exported_when_below_threshold() {
    let policy = SpanPolicy::default();
    let attrs = vec![("internal.version", "1.2.3", DataClass::Internal)];
    let exported = apply_span_policy(&policy, &attrs);
    // Internal is below Sensitive threshold so it is allowed.
    assert_eq!(exported[0].1, "1.2.3");
}
