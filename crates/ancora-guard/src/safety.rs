use crate::guardrail::{OutputGuardrail, GuardrailOutcome};

const UNSAFE_PATTERNS: &[&str] = &["<script", "DROP TABLE", "rm -rf", "HARM:"];

/// Blocks output containing unsafe or toxic content markers.
pub struct SafetyOutputGuardrail;

impl OutputGuardrail for SafetyOutputGuardrail {
    fn check_output(&self, output: &str) -> GuardrailOutcome {
        for pattern in UNSAFE_PATTERNS {
            if output.to_lowercase().contains(&pattern.to_lowercase()) {
                return GuardrailOutcome::Block(format!("blocked: pattern '{pattern}' detected"));
            }
        }
        GuardrailOutcome::Pass
    }
}
