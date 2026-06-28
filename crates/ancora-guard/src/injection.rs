use crate::guardrail::{InputGuardrail, GuardrailOutcome};

const INJECTION_PATTERNS: &[&str] = &[
    "ignore previous instructions",
    "system prompt:",
    "jailbreak",
    "disregard all",
];

/// Blocks inputs that appear to be jailbreak or injection attempts.
pub struct InjectionInputGuardrail;

impl InputGuardrail for InjectionInputGuardrail {
    fn check_input(&self, input: &str) -> GuardrailOutcome {
        let lower = input.to_lowercase();
        for pattern in INJECTION_PATTERNS {
            if lower.contains(pattern) {
                return GuardrailOutcome::Block(format!("injection attempt: '{pattern}'"));
            }
        }
        GuardrailOutcome::Pass
    }
}
