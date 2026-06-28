use crate::redact_parity::{
    reference_redact_result, check_redact_parity, assert_no_pii, standard_rules, RedactResult,
    reference_text,
};

#[test]
fn test_standard_rules_count() {
    let rules = standard_rules();
    assert_eq!(rules.len(), 4, "expected 4 standard redaction rules");
}

#[test]
fn test_redact_removes_email_domain() {
    let result = reference_redact_result("rust");
    assert!(
        !result.redacted_text.contains("@example.com"),
        "email domain should be redacted"
    );
}

#[test]
fn test_redact_removes_api_key_prefix() {
    let result = reference_redact_result("python");
    assert!(
        !result.redacted_text.contains("sk-"),
        "api key prefix should be redacted"
    );
}

#[test]
fn test_no_pii_after_redaction() {
    let result = reference_redact_result("typescript");
    let pii_issues = assert_no_pii(&result);
    assert!(pii_issues.is_empty(), "PII still present: {:?}", pii_issues);
}

#[test]
fn test_redact_parity_across_languages() {
    let langs = &["rust", "python", "typescript", "go", "java", "csharp"];
    let results: Vec<_> = langs.iter().map(|l| reference_redact_result(*l)).collect();
    let issues = check_redact_parity(&results);
    assert!(issues.is_empty(), "redact parity issues: {:?}", issues);
}

#[test]
fn test_redact_result_captures_rules_applied() {
    let rules = standard_rules();
    let result = RedactResult::new("rust", reference_text(), &rules);
    assert!(!result.rules_applied.is_empty(), "at least one rule should be applied");
}
