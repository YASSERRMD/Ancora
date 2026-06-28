use crate::custom::CustomInputGuardrail;
use crate::guardrail::{InputGuardrail, GuardrailOutcome};

#[test]
fn custom_guardrail_runs() {
    let g = CustomInputGuardrail::new("no_empty", |input| {
        if input.trim().is_empty() {
            GuardrailOutcome::Block("empty input".into())
        } else {
            GuardrailOutcome::Pass
        }
    });
    assert!(matches!(g.check_input("   "), GuardrailOutcome::Block(_)));
    assert_eq!(g.check_input("hello"), GuardrailOutcome::Pass);
}

#[test]
fn custom_guardrail_name_accessible() {
    let g = CustomInputGuardrail::new("length_check", |_| GuardrailOutcome::Pass);
    assert_eq!(g.name, "length_check");
}
