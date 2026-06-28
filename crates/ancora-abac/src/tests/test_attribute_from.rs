use crate::AttributeValue;
#[test]
fn from_str_ref() { assert!(matches!(AttributeValue::from("hello"), AttributeValue::String(_))); }
#[test]
fn from_i64() { assert!(matches!(AttributeValue::from(42i64), AttributeValue::Integer(42))); }
#[test]
fn from_bool() { assert!(matches!(AttributeValue::from(true), AttributeValue::Bool(true))); }
