use crate::guardrail::{GuardrailOutcome, InputGuardrail};

const PII_PATTERNS: &[&str] = &["@", "ssn:", "dob:", "credit_card:", "phone:"];

/// Redacts or blocks input containing PII markers.
pub struct PiiInputGuardrail;

impl InputGuardrail for PiiInputGuardrail {
    fn check_input(&self, input: &str) -> GuardrailOutcome {
        for pattern in PII_PATTERNS {
            if input.contains(pattern) {
                let redacted = input.replace(pattern, "[REDACTED]");
                return GuardrailOutcome::Repair(redacted);
            }
        }
        GuardrailOutcome::Pass
    }
}
