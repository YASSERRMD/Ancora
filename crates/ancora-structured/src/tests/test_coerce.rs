use crate::coerce::{coerce_bool, coerce_number, coerce_string};
use serde_json::json;

#[test]
fn coerce_number_from_string() {
    assert_eq!(coerce_number(&json!("3.14")), Some(3.14));
}

#[test]
fn coerce_bool_from_string() {
    assert_eq!(coerce_bool(&json!("true")), Some(true));
    assert_eq!(coerce_bool(&json!("no")), Some(false));
}

#[test]
fn coerce_string_from_number() {
    let s = coerce_string(&json!(42));
    assert!(s.is_some());
}

#[test]
fn coerce_number_from_invalid_string_is_none() {
    assert_eq!(coerce_number(&json!("not-a-number")), None);
}
