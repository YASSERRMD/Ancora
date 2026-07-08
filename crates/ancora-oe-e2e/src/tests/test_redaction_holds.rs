use crate::privacy_e2e::{assert_no_sensitive_data, default_redactor, RedactionPattern, Redactor};
use std::collections::HashMap;

#[test]
fn telemetry_redaction_holds() {
    let redactor = default_redactor();
    let text_with_pii = "Contact user@example.com or call +1-555-0100 using key sk-secret-12345.";

    let redacted = redactor.redact(text_with_pii);

    assert!(
        !redacted.contains("user@example.com"),
        "email must be redacted"
    );
    assert!(!redacted.contains("+1-555-0100"), "phone must be redacted");
    assert!(
        !redacted.contains("sk-secret-12345"),
        "api key must be redacted"
    );
    assert!(redacted.contains("[REDACTED_EMAIL]"));
    assert!(redacted.contains("[REDACTED_PHONE]"));
    assert!(redacted.contains("[REDACTED_KEY]"));
}

#[test]
fn assert_no_sensitive_data_passes_clean_text() {
    let redactor = default_redactor();
    let clean = "The result is 42 and everything looks good.";
    let result = assert_no_sensitive_data(&redactor, clean);
    assert!(result.is_ok());
}

#[test]
fn has_sensitive_data_detects_pii() {
    let redactor = default_redactor();
    assert!(redactor.has_sensitive_data("Email: user@example.com"));
    assert!(!redactor.has_sensitive_data("No PII here at all."));
}

#[test]
fn redact_map_redacts_attribute_values() {
    let redactor = default_redactor();
    let mut attrs = HashMap::new();
    attrs.insert("user.email".to_string(), "user@example.com".to_string());
    attrs.insert("model".to_string(), "local-judge".to_string());

    let redacted = redactor.redact_map(&attrs);
    assert_eq!(redacted["user.email"], "[REDACTED_EMAIL]");
    assert_eq!(redacted["model"], "local-judge");
}

#[test]
fn custom_pattern_redacts_correctly() {
    let mut redactor = Redactor::new();
    redactor.add_pattern(RedactionPattern::new("token", "MY_SECRET_TOKEN", "[TOKEN]"));

    let out = redactor.redact("Auth: MY_SECRET_TOKEN valid.");
    assert!(!out.contains("MY_SECRET_TOKEN"));
    assert!(out.contains("[TOKEN]"));
}
