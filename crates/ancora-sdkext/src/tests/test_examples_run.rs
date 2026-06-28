use std::collections::HashMap;
use std::sync::Arc;

use crate::examples_go::{GoEchoToolAdapter, GoWordCountAdapter};
use crate::examples_py::{make_py_echo_tool, make_py_sentiment_tool, PyToolWrapper};
use crate::examples_rs::{AddTool, EchoTool, KvStoreTool};
use crate::parity::InteropKit;
use crate::registration::{
    register_go_extension, register_python_extension, register_rust_extension, ExtensionRegistry,
};
use crate::rs_traits::{ToolExtension, Value};

// ---------------------------------------------------------------------------
// Rust examples
// ---------------------------------------------------------------------------

#[test]
fn test_rs_echo_tool_runs() {
    let tool = EchoTool;
    let mut args = HashMap::new();
    args.insert("message".to_string(), Value::string("hello world"));
    let result = tool.execute(args).unwrap();
    assert_eq!(result, Value::string("hello world"));
}

#[test]
fn test_rs_add_tool_runs() {
    let tool = AddTool;
    let mut args = HashMap::new();
    args.insert("a".to_string(), Value::Int(3));
    args.insert("b".to_string(), Value::Int(4));
    let result = tool.execute(args).unwrap();
    assert_eq!(result, Value::Int(7));
}

#[test]
fn test_rs_kv_store_set_get() {
    let tool = KvStoreTool::new();
    let mut set_args = HashMap::new();
    set_args.insert("op".to_string(), Value::string("set"));
    set_args.insert("key".to_string(), Value::string("color"));
    set_args.insert("value".to_string(), Value::string("blue"));
    tool.execute(set_args).unwrap();

    let mut get_args = HashMap::new();
    get_args.insert("op".to_string(), Value::string("get"));
    get_args.insert("key".to_string(), Value::string("color"));
    let result = tool.execute(get_args).unwrap();
    assert_eq!(result, Value::string("blue"));
}

#[test]
fn test_rs_kv_store_get_missing_returns_null() {
    let tool = KvStoreTool::new();
    let mut get_args = HashMap::new();
    get_args.insert("op".to_string(), Value::string("get"));
    get_args.insert("key".to_string(), Value::string("nope"));
    let result = tool.execute(get_args).unwrap();
    assert_eq!(result, Value::Null);
}

#[test]
fn test_rs_examples_pass_interop_kit() {
    let tools: Vec<Box<dyn ToolExtension>> = vec![
        Box::new(EchoTool),
        Box::new(AddTool),
        Box::new(KvStoreTool::new()),
    ];
    for tool in &tools {
        let results = InteropKit::run_all(tool.as_ref());
        for r in &results {
            assert!(r.passed, "[rs] check '{}' failed: {}", r.name, r.message);
        }
    }
}

// ---------------------------------------------------------------------------
// Go examples
// ---------------------------------------------------------------------------

#[test]
fn test_go_echo_tool_runs() {
    let tool = GoEchoToolAdapter::new();
    let mut args = HashMap::new();
    args.insert("message".to_string(), Value::string("ping"));
    let result = tool.execute(args).unwrap();
    if let Value::Str(s) = result {
        assert!(s.contains("ping"));
    } else {
        panic!("expected Str");
    }
}

#[test]
fn test_go_word_count_runs() {
    let tool = GoWordCountAdapter::new();
    let mut args = HashMap::new();
    args.insert("text".to_string(), Value::string("one two three"));
    let result = tool.execute(args).unwrap();
    assert_eq!(result, Value::Int(3));
}

#[test]
fn test_go_examples_pass_interop_kit() {
    let tools: Vec<Box<dyn ToolExtension>> = vec![
        Box::new(GoEchoToolAdapter::new()),
        Box::new(GoWordCountAdapter::new()),
    ];
    for tool in &tools {
        let results = InteropKit::run_all(tool.as_ref());
        for r in &results {
            assert!(r.passed, "[go] check '{}' failed: {}", r.name, r.message);
        }
    }
}

// ---------------------------------------------------------------------------
// Python examples
// ---------------------------------------------------------------------------

#[test]
fn test_py_echo_tool_runs() {
    let adapter = make_py_echo_tool();
    let mut args = HashMap::new();
    args.insert("message".to_string(), Value::string("hi"));
    let result = adapter.execute(args).unwrap();
    if let Value::Str(s) = result {
        assert!(s.contains("hi"));
    } else {
        panic!("expected Str");
    }
}

#[test]
fn test_py_sentiment_tool_runs() {
    let adapter = make_py_sentiment_tool();
    let mut args = HashMap::new();
    args.insert("text".to_string(), Value::string("This is great news!"));
    let result = adapter.execute(args).unwrap();
    if let Value::Map(m) = result {
        assert!(m.contains_key("label"));
        let label = m["label"].as_str().unwrap();
        assert_eq!(label, "positive");
    } else {
        panic!("expected Map");
    }
}

#[test]
fn test_py_examples_pass_interop_kit() {
    let tools: Vec<Box<dyn ToolExtension>> = vec![
        Box::new(PyToolWrapper::new(make_py_echo_tool())),
        Box::new(PyToolWrapper::new(make_py_sentiment_tool())),
    ];
    for tool in &tools {
        let results = InteropKit::run_all(tool.as_ref());
        for r in &results {
            assert!(r.passed, "[py] check '{}' failed: {}", r.name, r.message);
        }
    }
}

// ---------------------------------------------------------------------------
// Registry integration: all examples registered and dispatched
// ---------------------------------------------------------------------------

#[test]
fn test_all_examples_in_registry() {
    let registry = ExtensionRegistry::new();

    register_rust_extension(&registry, Arc::new(EchoTool)).unwrap();
    register_rust_extension(&registry, Arc::new(AddTool)).unwrap();
    register_go_extension(&registry, Arc::new(GoEchoToolAdapter::new())).unwrap();
    register_go_extension(&registry, Arc::new(GoWordCountAdapter::new())).unwrap();
    register_python_extension(
        &registry,
        Arc::new(PyToolWrapper::new(make_py_echo_tool())),
    )
    .unwrap();

    assert!(registry.len() >= 5);

    // Dispatch echo
    let mut echo_args = HashMap::new();
    echo_args.insert("message".to_string(), Value::string("test"));
    let result = registry.dispatch("echo", echo_args);
    assert!(result.is_ok());
}
