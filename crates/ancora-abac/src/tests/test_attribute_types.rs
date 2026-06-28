use crate::AttributeValue;
#[test]
fn bool_value() { assert_eq!(AttributeValue::Bool(true).as_bool(), Some(true)); }
#[test]
fn list_contains() { assert!(AttributeValue::List(vec!["a".into(),"b".into()]).contains("a")); }
#[test]
fn list_not_contains() { assert!(!AttributeValue::List(vec!["a".into()]).contains("z")); }
