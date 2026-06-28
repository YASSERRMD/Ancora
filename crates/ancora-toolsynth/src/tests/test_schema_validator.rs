use crate::schema_validator::SchemaValidator;
use serde_json::json;

#[test]
fn valid_object_schema_passes() {
    let schema = json!({ "type": "object", "properties": {} });
    assert!(SchemaValidator::validate(&schema).is_ok());
}

#[test]
fn valid_string_schema_passes() {
    let schema = json!({ "type": "string" });
    assert!(SchemaValidator::validate(&schema).is_ok());
}

#[test]
fn unknown_type_fails() {
    let schema = json!({ "type": "dict" });
    assert!(SchemaValidator::validate(&schema).is_err());
}

#[test]
fn missing_type_fails() {
    let schema = json!({ "properties": {} });
    assert!(SchemaValidator::validate(&schema).is_err());
}

#[test]
fn non_object_schema_fails() {
    let schema = json!("just a string");
    assert!(SchemaValidator::validate(&schema).is_err());
}
