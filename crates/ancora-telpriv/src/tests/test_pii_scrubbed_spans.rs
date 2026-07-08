use crate::classification::DataClass;
use crate::span_policy::{apply_span_policy, SpanPolicy};

#[test]
fn pii_attributes_not_in_exported_span() {
    let policy = SpanPolicy::default();
    let attrs = vec![
        ("span.name", "invoke_tool", DataClass::Public),
        ("user.email", "alice@corp.com", DataClass::Sensitive),
        ("token.count", "42", DataClass::Internal),
    ];
    let exported = apply_span_policy(&policy, &attrs);

    // email must be redacted, not raw
    let email_entry = exported.iter().find(|(k, _)| k == "user.email");
    assert!(
        email_entry.is_some(),
        "email key should be present but redacted"
    );
    let (_, email_val) = email_entry.unwrap();
    assert_ne!(email_val, "alice@corp.com", "raw email must not appear");
    assert_eq!(email_val, "[REDACTED]");

    // span.name is safe
    let name_entry = exported.iter().find(|(k, _)| k == "span.name");
    assert!(name_entry.is_some());
    assert_eq!(name_entry.unwrap().1, "invoke_tool");
}

#[test]
fn prompt_prefix_dropped_entirely() {
    let policy = SpanPolicy::default();
    let attrs = vec![("prompt.text", "Tell me about Alice", DataClass::Public)];
    let exported = apply_span_policy(&policy, &attrs);
    assert!(
        exported.iter().all(|(k, _)| k != "prompt.text"),
        "prompt.text must be dropped"
    );
}

#[test]
fn critical_class_redacted() {
    let policy = SpanPolicy {
        redact_at_or_above: crate::span_policy::SpanPolicy::default().redact_at_or_above,
        drop_prefixes: vec![],
    };
    let attrs = vec![("api.key", "sk-abc123", DataClass::Critical)];
    let exported = apply_span_policy(&policy, &attrs);
    let entry = exported.iter().find(|(k, _)| k == "api.key").unwrap();
    assert_eq!(entry.1, "[REDACTED]");
}
