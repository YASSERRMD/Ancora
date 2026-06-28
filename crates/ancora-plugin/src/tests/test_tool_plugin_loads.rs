//! Tests: a sample tool plugin loads and executes correctly.

use std::collections::HashMap;
use crate::manifest::{ManifestBuilder, PluginKind, SemVer};
use crate::tool_ext::{EchoTool, ToolError, ToolInput, ToolPlugin, Value};

fn build_tool_manifest() -> crate::manifest::PluginManifest {
    ManifestBuilder::new()
        .id("echo-tool")
        .name("Echo Tool")
        .version(SemVer::new(1, 0, 0))
        .sdk_range(SemVer::new(1, 0, 0), SemVer::new(1, 99, 0))
        .kind(PluginKind::Tool)
        .scope("tool:execute")
        .build()
        .unwrap()
}

#[test]
fn echo_tool_has_correct_spec() {
    let t = EchoTool::new();
    let spec = t.spec();
    assert_eq!(spec.name, "echo");
    assert!(!spec.args.is_empty());
    assert_eq!(spec.args[0].name, "text");
}

#[test]
fn echo_tool_returns_input_text() {
    let t = EchoTool::new();
    let mut args = HashMap::new();
    args.insert("text".to_string(), Value::Str("ping".to_string()));
    let out = t.call(ToolInput { tool_name: "echo".to_string(), args }).unwrap();
    assert_eq!(out.value, Value::Str("ping".to_string()));
}

#[test]
fn echo_tool_missing_arg_returns_error() {
    let t = EchoTool::new();
    let err = t.call(ToolInput {
        tool_name: "echo".to_string(),
        args: HashMap::new(),
    }).unwrap_err();
    assert_eq!(err, ToolError::MissingArg("text".into()));
}

#[test]
fn tool_manifest_is_valid() {
    let m = build_tool_manifest();
    assert_eq!(m.kind, PluginKind::Tool);
    assert!(m.required_scopes.contains(&"tool:execute".to_string()));
}
