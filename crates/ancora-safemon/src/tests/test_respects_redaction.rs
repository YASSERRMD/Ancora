use crate::pii::PiiDetector;

#[test]
fn email_redacted_from_output() {
    let d = PiiDetector::new();
    let original = "Contact admin@secret.com for more info.";
    let redacted = d.redact(original);
    assert!(!redacted.contains("admin@secret.com"));
    assert!(redacted.contains("[REDACTED_EMAIL]"));
}

#[test]
fn ssn_redacted_from_output() {
    let d = PiiDetector::new();
    let original = "The SSN is 111-22-3333 per the form.";
    let redacted = d.redact(original);
    assert!(!redacted.contains("111-22-3333"));
    assert!(redacted.contains("[REDACTED_SSN]"));
}

#[test]
fn text_without_pii_unchanged() {
    let d = PiiDetector::new();
    let original = "No sensitive data here.";
    let redacted = d.redact(original);
    assert_eq!(redacted, original);
}

#[test]
fn multiple_emails_all_redacted() {
    let d = PiiDetector::new();
    let original = "Send to alice@foo.com and bob@bar.com";
    let redacted = d.redact(original);
    assert!(!redacted.contains("alice@foo.com"));
    assert!(redacted.contains("[REDACTED_EMAIL]"));
}

#[test]
fn redacted_output_preserves_non_pii_content() {
    let d = PiiDetector::new();
    let original = "Dear customer user@example.com, your order is ready.";
    let redacted = d.redact(original);
    assert!(redacted.contains("Dear customer"));
    assert!(redacted.contains("your order is ready."));
}
