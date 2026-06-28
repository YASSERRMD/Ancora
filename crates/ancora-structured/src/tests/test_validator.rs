use crate::schema::{FieldSchema, JsonType, OutputSchema};
use crate::validator::OutputValidator;
use serde_json::json;

fn schema() -> OutputSchema {
    OutputSchema::new("task")
        .add_field(FieldSchema::new("title", JsonType::String, true))
        .add_field(FieldSchema::new("score", JsonType::Number, false))
}

#[test]
fn valid_object_passes() {
    let s = schema();
    let v = json!({"title": "test", "score": 9.5});
    assert!(OutputValidator::validate(&s, &v).is_ok());
}

#[test]
fn missing_required_field_fails() {
    let s = schema();
    let v = json!({"score": 9.5});
    assert!(OutputValidator::validate(&s, &v).is_err());
}

#[test]
fn wrong_type_fails() {
    let s = schema();
    let v = json!({"title": 42}); // title should be string
    assert!(OutputValidator::validate(&s, &v).is_err());
}

#[test]
fn not_object_fails() {
    let s = schema();
    let v = json!("just a string");
    assert!(OutputValidator::validate(&s, &v).is_err());
}

#[test]
fn optional_missing_field_passes() {
    let s = schema();
    let v = json!({"title": "t"});
    assert!(OutputValidator::validate(&s, &v).is_ok());
}
