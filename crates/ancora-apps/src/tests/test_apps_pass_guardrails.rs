use crate::safety::{GuardrailOutcome, SafetyGuardrail};

#[test]
fn document_qa_output_passes_guardrails() {
    let guard = SafetyGuardrail::default_rules();
    let qa_output = "The retention policy requires records to be kept for 7 years.";
    assert!(guard.is_safe(qa_output));
    assert_eq!(guard.evaluate(qa_output), GuardrailOutcome::Allow);
}

#[test]
fn research_output_passes_guardrails() {
    let guard = SafetyGuardrail::default_rules();
    let output = "Zero Trust is a security model that verifies every access request.";
    assert!(guard.is_safe(output));
}

#[test]
fn coding_dangerous_output_is_blocked() {
    let guard = SafetyGuardrail::default_rules();
    let dangerous = "Run this command: rm -rf /var/data";
    let outcome = guard.evaluate(dangerous);
    assert!(matches!(outcome, GuardrailOutcome::Block { .. }));
    assert!(!guard.is_safe(dangerous));
}

#[test]
fn compliance_output_with_secret_is_redacted() {
    let guard = SafetyGuardrail::default_rules();
    // The pattern "SECRET=" is replaced; text after the pattern is unchanged
    // but the sensitive keyword trigger is gone.
    let text = "Config value: SECRET=mypassword123";
    let outcome = guard.evaluate(text);
    assert!(matches!(outcome, GuardrailOutcome::Redact { .. }));
    if let GuardrailOutcome::Redact { redacted } = outcome {
        // The literal pattern "SECRET=" must be gone.
        assert!(!redacted.contains("SECRET="), "pattern should be replaced");
        assert!(redacted.contains("[REDACTED]"));
    }
}

#[test]
fn sql_injection_is_blocked() {
    let guard = SafetyGuardrail::default_rules();
    let sqli = "'; DROP TABLE users; --";
    assert!(!guard.is_safe(sqli));
}

#[test]
fn xss_is_blocked() {
    let guard = SafetyGuardrail::default_rules();
    let xss = "Hello <script>alert('xss')</script>";
    assert!(!guard.is_safe(xss));
}
