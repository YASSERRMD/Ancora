use crate::rs_traits::{ExtensionError, ToolMeta, Value};
use crate::ts_interfaces::{canonical_ts_interface, validate_ts_value, TsExtensionAdapter, TsType};
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn test_ts_type_annotations() {
    assert_eq!(TsType::String.annotation(), "string");
    assert_eq!(TsType::Number.annotation(), "number");
    assert_eq!(TsType::Boolean.annotation(), "boolean");
    assert_eq!(
        TsType::Array(Box::new(TsType::String)).annotation(),
        "string[]"
    );
    assert_eq!(
        TsType::Record(Box::new(TsType::String), Box::new(TsType::Number)).annotation(),
        "Record<string, number>"
    );
}

#[test]
fn test_canonical_ts_interface() {
    let iface = canonical_ts_interface();
    assert_eq!(iface.interface_name, "IToolExtension");
    assert!(!iface.methods.is_empty());
    // "execute" method must be async
    let execute = iface.methods.iter().find(|m| m.name == "execute");
    assert!(execute.is_some());
    assert!(execute.unwrap().is_async);
}

#[test]
fn test_validate_ts_value_string() {
    assert!(validate_ts_value(&Value::string("hello"), &TsType::String));
    assert!(!validate_ts_value(&Value::Int(1), &TsType::String));
}

#[test]
fn test_validate_ts_value_number() {
    assert!(validate_ts_value(&Value::Int(42), &TsType::Number));
    assert!(validate_ts_value(&Value::Float(3.25), &TsType::Number));
    assert!(!validate_ts_value(&Value::string("x"), &TsType::Number));
}

#[test]
fn test_validate_ts_value_any_matches_all() {
    assert!(validate_ts_value(&Value::Null, &TsType::Any));
    assert!(validate_ts_value(&Value::Int(1), &TsType::Any));
    assert!(validate_ts_value(&Value::string("s"), &TsType::Any));
}

#[test]
fn test_ts_adapter_execute() {
    let meta = ToolMeta::new("ts_reverse", "Reverses a string.", "1.0.0");
    let adapter = TsExtensionAdapter::new(meta, |args| {
        let s = args
            .get("text")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ExtensionError::InvalidArgument("'text' required".to_string()))?;
        Ok(Value::string(s.chars().rev().collect::<String>()))
    });

    let mut args = HashMap::new();
    args.insert("text".to_string(), Value::string("abc"));
    let result = adapter.execute(args).unwrap();
    assert_eq!(result, Value::string("cba"));
}

#[test]
fn test_ts_adapter_meta() {
    let meta = ToolMeta::new("ts_tool", "desc", "0.1.0");
    let adapter = TsExtensionAdapter::new(meta, |_| Ok(Value::Null));
    assert_eq!(adapter.meta().name, "ts_tool");
}
