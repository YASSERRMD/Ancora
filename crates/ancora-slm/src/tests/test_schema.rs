use crate::schema::{
    augment_prompt_with_schema, schema_to_prompt_hint, validate, Schema,
};
use serde_json::json;

#[test]
fn test_schema_validates_correct_object() {
    let schema = Schema::Object {
        required: vec!["name".into(), "age".into()],
        properties: vec![
            ("name".into(), Schema::String),
            ("age".into(), Schema::Number),
        ],
    };
    let value = json!({"name": "Alice", "age": 30});
    let errors = validate(&value, &schema);
    assert!(errors.is_empty(), "expected no validation errors, got {:?}", errors);
}

#[test]
fn test_schema_catches_missing_required_field() {
    let schema = Schema::Object {
        required: vec!["id".into()],
        properties: vec![("id".into(), Schema::String)],
    };
    let value = json!({"other": "value"});
    let errors = validate(&value, &schema);
    assert!(
        errors.iter().any(|e| e.message.contains("missing required property 'id'")),
        "expected missing-field error, got {:?}",
        errors
    );
}

#[test]
fn test_schema_catches_wrong_type() {
    let schema = Schema::Object {
        required: vec!["count".into()],
        properties: vec![("count".into(), Schema::Number)],
    };
    let value = json!({"count": "not-a-number"});
    let errors = validate(&value, &schema);
    assert!(
        !errors.is_empty(),
        "expected type-mismatch error for string in Number field"
    );
}

#[test]
fn test_schema_validates_enum() {
    let schema = Schema::Enum(vec!["red".into(), "green".into(), "blue".into()]);
    assert!(validate(&json!("red"), &schema).is_empty());
    let errs = validate(&json!("purple"), &schema);
    assert!(!errs.is_empty(), "expected enum violation for 'purple'");
}

#[test]
fn test_schema_validates_array_items() {
    let schema = Schema::Array { item_schema: Box::new(Schema::Number) };
    assert!(validate(&json!([1, 2, 3]), &schema).is_empty());
    let errs = validate(&json!([1, "two", 3]), &schema);
    assert!(!errs.is_empty(), "expected type error for string item in number array");
}

#[test]
fn test_schema_to_prompt_hint_object() {
    let schema = Schema::Object {
        required: vec!["city".into()],
        properties: vec![("city".into(), Schema::String)],
    };
    let hint = schema_to_prompt_hint(&schema);
    assert!(hint.contains("city"), "hint should mention required field 'city'");
}

#[test]
fn test_augment_prompt_with_schema() {
    let schema = Schema::Object {
        required: vec!["answer".into()],
        properties: vec![("answer".into(), Schema::String)],
    };
    let prompt = "What is the capital of France?";
    let augmented = augment_prompt_with_schema(prompt, &schema);
    assert!(augmented.contains(prompt), "original prompt should be retained");
    assert!(augmented.contains("Output format:"), "should include output format hint");
}
