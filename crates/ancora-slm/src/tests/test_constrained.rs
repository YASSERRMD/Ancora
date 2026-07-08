use crate::constrained::{
    add_json_fence_instruction, extract_json, run_constrained, validate_json_output,
    ConstrainedConfig, ConstrainedError,
};
use serde_json::Value;

#[test]
fn test_extract_json_from_fenced_block() {
    let raw = "Sure!\n```json\n{\"key\": 42}\n```\nDone.";
    let extracted = extract_json(raw).expect("should extract JSON");
    let v: Value = serde_json::from_str(&extracted).expect("should parse");
    assert_eq!(v["key"], 42);
}

#[test]
fn test_extract_json_bare_object() {
    let raw = r#"{"foo": "bar"}"#;
    let extracted = extract_json(raw).expect("should extract bare JSON");
    assert!(extracted.contains("foo"));
}

#[test]
fn test_validate_json_output_valid() {
    let raw = r#"{"answer": 42}"#;
    let v = validate_json_output(raw).expect("should validate");
    assert_eq!(v["answer"], 42);
}

#[test]
fn test_validate_json_output_invalid() {
    let raw = "not json at all";
    let err = validate_json_output(raw).unwrap_err();
    assert!(matches!(err, ConstrainedError::NotJson(_)));
}

#[test]
fn test_constrained_decoding_yields_valid_json_on_first_try() {
    let config = ConstrainedConfig {
        max_retries: 2,
        add_json_fence_instruction: false,
    };
    let model_fn = |_: &str| -> String { r#"{"result": "ok"}"#.to_string() };
    let v = run_constrained("Give me JSON", &config, model_fn).expect("should succeed");
    assert_eq!(v["result"], "ok");
}

#[test]
fn test_constrained_decoding_retries_on_failure() {
    use std::cell::Cell;
    let config = ConstrainedConfig {
        max_retries: 3,
        add_json_fence_instruction: false,
    };
    let call_count = Cell::new(0usize);
    // First two calls return prose, third returns valid JSON.
    let model_fn = |_: &str| -> String {
        let n = call_count.get() + 1;
        call_count.set(n);
        if n < 3 {
            "I cannot produce JSON right now.".to_string()
        } else {
            r#"{"done": true}"#.to_string()
        }
    };
    let v = run_constrained("Task", &config, model_fn).expect("should succeed after retries");
    assert_eq!(v["done"], true);
}

#[test]
fn test_json_fence_instruction_added_to_prompt() {
    let prompt = "Extract entities.";
    let augmented = add_json_fence_instruction(prompt);
    assert!(
        augmented.contains("Extract entities."),
        "original prompt should be retained"
    );
    assert!(
        augmented.contains("```json"),
        "should include JSON fence hint"
    );
}
