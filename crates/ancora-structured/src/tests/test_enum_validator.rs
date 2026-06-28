use crate::enum_validator::EnumValidator;
use serde_json::json;

#[test]
fn valid_enum_value_passes() {
    let v = EnumValidator::new("category", vec!["bug", "feature"]);
    assert!(v.validate(&json!("bug")).is_ok());
}

#[test]
fn invalid_enum_value_fails() {
    let v = EnumValidator::new("category", vec!["bug", "feature"]);
    assert!(v.validate(&json!("unknown")).is_err());
}

#[test]
fn non_string_fails() {
    let v = EnumValidator::new("category", vec!["bug"]);
    assert!(v.validate(&json!(42)).is_err());
}
