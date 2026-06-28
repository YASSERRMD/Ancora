use crate::pii::PiiInputGuardrail;
use crate::guardrail::{InputGuardrail, GuardrailOutcome};

#[test]
fn pii_input_blocked_or_redacted() {
    let g = PiiInputGuardrail;
    let result = g.check_input("send email to user@example.com");
    assert!(matches!(result, GuardrailOutcome::Repair(_)));
}

#[test]
fn clean_input_passes() {
    let g = PiiInputGuardrail;
    assert_eq!(g.check_input("summarize this document"), GuardrailOutcome::Pass);
}

#[test]
fn ssn_marker_redacted() {
    let g = PiiInputGuardrail;
    let result = g.check_input("ssn: 123-45-6789");
    assert!(matches!(result, GuardrailOutcome::Repair(_)));
    if let GuardrailOutcome::Repair(s) = result {
        assert!(s.contains("[REDACTED]"));
    }
}
