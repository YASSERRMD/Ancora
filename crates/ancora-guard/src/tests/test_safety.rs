use crate::safety::SafetyOutputGuardrail;
use crate::guardrail::{OutputGuardrail, GuardrailOutcome};

#[test]
fn unsafe_output_blocked() {
    let g = SafetyOutputGuardrail;
    let result = g.check_output("Here is <script>alert(1)</script>");
    assert!(matches!(result, GuardrailOutcome::Block(_)));
}

#[test]
fn safe_output_passes() {
    let g = SafetyOutputGuardrail;
    assert_eq!(g.check_output("The answer is 42."), GuardrailOutcome::Pass);
}

#[test]
fn drop_table_blocked() {
    let g = SafetyOutputGuardrail;
    assert!(matches!(g.check_output("DROP TABLE users;"), GuardrailOutcome::Block(_)));
}
