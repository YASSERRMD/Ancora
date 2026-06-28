use crate::schema_guard::SchemaOutputGuardrail;
use crate::guardrail::{OutputGuardrail, GuardrailOutcome};

#[test]
fn valid_json_object_passes() {
    let g = SchemaOutputGuardrail;
    assert_eq!(g.check_output(r#"{"key": "value"}"#), GuardrailOutcome::Pass);
}

#[test]
fn malformed_output_repaired() {
    let g = SchemaOutputGuardrail;
    let result = g.check_output("plain text output");
    assert!(matches!(result, GuardrailOutcome::Repair(_)));
}

#[test]
fn whitespace_around_braces_passes() {
    let g = SchemaOutputGuardrail;
    assert_eq!(g.check_output(r#"  {"a": 1}  "#), GuardrailOutcome::Pass);
}
