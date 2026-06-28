use crate::tool_template::{MyTool, ToolError, ToolPlugin, Value};
use std::collections::HashMap;

fn args(pairs: &[(&str, &str)]) -> HashMap<String, Value> {
    pairs.iter().map(|(k, v)| (k.to_string(), Value::Text(v.to_string()))).collect()
}

#[test]
fn tool_id_is_correct() {
    let t = MyTool;
    assert_eq!(t.tool_id(), "my-tool");
}

#[test]
fn description_is_nonempty() {
    assert!(!MyTool.description().is_empty());
}

#[test]
fn params_schema_has_two_entries() {
    let params = MyTool.params();
    assert_eq!(params.len(), 2);
    let names: Vec<&str> = params.iter().map(|p| p.name.as_str()).collect();
    assert!(names.contains(&"left"));
    assert!(names.contains(&"right"));
}

#[test]
fn execute_concatenates_strings() {
    let result = MyTool.execute(args(&[("left", "foo"), ("right", "bar")])).unwrap();
    assert_eq!(result, Value::Text("foobar".to_string()));
}

#[test]
fn execute_empty_strings() {
    let result = MyTool.execute(args(&[("left", ""), ("right", "")])).unwrap();
    assert_eq!(result, Value::Text("".to_string()));
}

#[test]
fn execute_missing_left_returns_error() {
    let result = MyTool.execute(args(&[("right", "bar")]));
    match result {
        Err(ToolError::MissingArgument(n)) => assert_eq!(n, "left"),
        other => panic!("expected MissingArgument(left), got {other:?}"),
    }
}

#[test]
fn execute_missing_right_returns_error() {
    let result = MyTool.execute(args(&[("left", "foo")]));
    match result {
        Err(ToolError::MissingArgument(n)) => assert_eq!(n, "right"),
        other => panic!("expected MissingArgument(right), got {other:?}"),
    }
}

#[test]
fn execute_wrong_type_returns_error() {
    // Passing a number where a string is expected.
    let mut a = HashMap::new();
    a.insert("left".to_string(), Value::Number(42.0));
    a.insert("right".to_string(), Value::Text("ok".to_string()));
    let result = MyTool.execute(a);
    match result {
        Err(ToolError::MissingArgument(n)) => assert_eq!(n, "left"),
        other => panic!("expected MissingArgument(left) for wrong type, got {other:?}"),
    }
}

#[test]
fn value_helpers() {
    assert_eq!(Value::Text("hi".into()).as_str(), Some("hi"));
    assert_eq!(Value::Number(3.14).as_f64(), Some(3.14));
    assert_eq!(Value::Bool(true).as_bool(), Some(true));
    assert_eq!(Value::Null.as_str(), None);
}

#[test]
fn tool_error_display() {
    assert!(!ToolError::MissingArgument("x".into()).to_string().is_empty());
    let e = ToolError::InvalidArgument { name: "a".into(), reason: "bad".into() };
    assert!(e.to_string().contains("a") && e.to_string().contains("bad"));
}
