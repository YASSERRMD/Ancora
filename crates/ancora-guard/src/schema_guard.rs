use crate::guardrail::{OutputGuardrail, GuardrailOutcome};

/// Validates that output starts and ends with JSON braces.
pub struct SchemaOutputGuardrail;

impl OutputGuardrail for SchemaOutputGuardrail {
    fn check_output(&self, output: &str) -> GuardrailOutcome {
        let trimmed = output.trim();
        if trimmed.starts_with('{') && trimmed.ends_with('}') {
            GuardrailOutcome::Pass
        } else {
            let repaired = format!("{{{}}}", trimmed.trim_matches(|c| c == '{' || c == '}'));
            GuardrailOutcome::Repair(repaired)
        }
    }
}
