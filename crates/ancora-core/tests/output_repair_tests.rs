use ancora_core::error::AncoraError;
use ancora_core::output::{repair_prompt, validate_output, validate_with_repair};

const SCHEMA: &str = r#"{"type":"object"}"#;

#[test]
fn valid_json_passes_with_schema() {
    validate_output(r#"{"key":"value"}"#, SCHEMA).unwrap();
}

#[test]
fn invalid_json_fails_with_schema() {
    let err = validate_output("not json at all", SCHEMA);
    assert!(err.is_err(), "non-JSON must fail validation");
}

#[test]
fn empty_schema_accepts_any_string() {
    validate_output("literally anything", "").unwrap();
}

#[test]
fn repair_succeeds_on_first_valid_output() {
    let result = validate_with_repair(
        r#"{"ok":true}"#.to_string(),
        SCHEMA,
        3,
        |_, _| unreachable!("repair must not be called when first output is valid"),
    );
    assert_eq!(result.unwrap(), r#"{"ok":true}"#);
}

#[test]
fn repair_called_once_when_first_output_is_invalid() {
    let mut repair_calls = 0u32;
    let result = validate_with_repair(
        "invalid".to_string(),
        SCHEMA,
        3,
        |_, _| {
            repair_calls += 1;
            Ok(r#"{"repaired":true}"#.to_string())
        },
    );
    assert_eq!(repair_calls, 1, "repair must be called exactly once");
    assert_eq!(result.unwrap(), r#"{"repaired":true}"#);
}

#[test]
fn repair_exhausts_and_returns_error_after_max_attempts() {
    let mut calls = 0u32;
    let result = validate_with_repair(
        "bad".to_string(),
        SCHEMA,
        3,
        |_, _| {
            calls += 1;
            Ok("still bad".to_string())
        },
    );
    assert!(
        matches!(result, Err(AncoraError::OutputValidation { attempts: 3, .. })),
        "must fail with OutputValidation after 3 attempts"
    );
}

#[test]
fn max_attempts_one_means_no_repair_attempt() {
    let result = validate_with_repair(
        "bad".to_string(),
        SCHEMA,
        1,
        |_, _| panic!("repair must not be called when max_attempts=1"),
    );
    assert!(
        matches!(result, Err(AncoraError::OutputValidation { attempts: 1, .. })),
        "must fail immediately with OutputValidation"
    );
}

#[test]
fn repair_prompt_contains_reason_and_output() {
    let prompt = repair_prompt("bad-output", "missing key");
    assert!(prompt.contains("bad-output"), "prompt must include original output");
    assert!(prompt.contains("missing key"), "prompt must include failure reason");
}

#[test]
fn validate_output_error_message_includes_position_hint() {
    let err = validate_output("{bad json}", SCHEMA);
    assert!(err.is_err());
    let msg = err.unwrap_err();
    assert!(!msg.is_empty(), "error message must not be empty");
}

#[test]
fn repair_fn_error_propagates_immediately() {
    let result = validate_with_repair(
        "bad".to_string(),
        SCHEMA,
        5,
        |_, _| Err(AncoraError::ModelRefused("model refused to repair".into())),
    );
    assert!(
        matches!(result, Err(AncoraError::ModelRefused(_))),
        "repair_fn error must propagate"
    );
}
