use crate::repair::{repair_tool_call, RepairError};

#[test]
fn test_repair_valid_json_passthrough() {
    let raw = r#"{"name": "get_weather", "arguments": {"city": "London"}}"#;
    let result = repair_tool_call(raw).expect("should repair valid JSON");
    assert_eq!(result.name, "get_weather");
    assert_eq!(result.arguments["city"], "London");
}

#[test]
fn test_repair_extracts_from_prose() {
    let raw =
        r#"Sure, I'll call the tool: {"name": "search", "arguments": {"query": "rust"}} Done."#;
    let result = repair_tool_call(raw).expect("should extract JSON from prose");
    assert_eq!(result.name, "search");
}

#[test]
fn test_repair_trailing_comma_fix() {
    let raw = r#"{"name": "calc", "arguments": {"x": 1,}}"#;
    let result = repair_tool_call(raw).expect("should fix trailing comma");
    assert_eq!(result.name, "calc");
    assert_eq!(result.arguments["x"], 1);
}

#[test]
fn test_repair_normalises_function_name_field() {
    let raw = r#"{"function_name": "lookup", "arguments": {}}"#;
    let result = repair_tool_call(raw).expect("should normalise function_name to name");
    assert_eq!(result.name, "lookup");
}

#[test]
fn test_repair_normalises_args_field() {
    let raw = r#"{"name": "send", "args": {"to": "alice"}}"#;
    let result = repair_tool_call(raw).expect("should normalise args to arguments");
    assert_eq!(result.arguments["to"], "alice");
}

#[test]
fn test_repair_no_json_returns_error() {
    let raw = "I don't know what to do here.";
    let result = repair_tool_call(raw);
    assert_eq!(result, Err(RepairError::NoJsonFound));
}

#[test]
fn test_repair_normalises_params_field() {
    let raw = r#"{"name": "fn1", "params": {"key": "value"}}"#;
    let result = repair_tool_call(raw).expect("should normalise params");
    assert_eq!(result.arguments["key"], "value");
}
