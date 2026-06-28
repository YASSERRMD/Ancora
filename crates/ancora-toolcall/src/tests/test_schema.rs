use crate::schema::{ToolCall, ToolDef, ToolResult};
use serde_json::json;

#[test]
fn tool_def_defaults() {
    let def = ToolDef::new("search", "search the web");
    assert_eq!(def.timeout_ms, 5000);
    assert!(!def.is_async);
}

#[test]
fn tool_result_ok() {
    let r = ToolResult::ok("c1", "search", json!("result"), 100);
    assert!(!r.is_error);
    assert_eq!(r.elapsed_ms, 100);
}

#[test]
fn tool_result_error() {
    let r = ToolResult::error("c2", "search", "timeout");
    assert!(r.is_error);
}

#[test]
fn tool_call_fields() {
    let c = ToolCall::new("id1", "search", json!({"q": "rust"}));
    assert_eq!(c.call_id, "id1");
    assert_eq!(c.tool_name, "search");
}
