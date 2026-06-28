use crate::result_merge::{error_count, merge_results, results_to_messages};
use crate::schema::ToolResult;
use serde_json::json;

#[test]
fn merge_results_creates_map() {
    let results = vec![
        ToolResult::ok("c1", "search", json!("data"), 10),
        ToolResult::error("c2", "exec", "fail"),
    ];
    let merged = merge_results(&results);
    assert!(merged.is_object());
    let obj = merged.as_object().unwrap();
    assert_eq!(obj.len(), 2);
}

#[test]
fn error_count_correct() {
    let results = vec![
        ToolResult::ok("c1", "a", json!(null), 0),
        ToolResult::error("c2", "b", "err"),
        ToolResult::error("c3", "c", "err2"),
    ];
    assert_eq!(error_count(&results), 2);
}

#[test]
fn results_to_messages_role_is_tool() {
    let results = vec![ToolResult::ok("c1", "search", json!("data"), 0)];
    let msgs = results_to_messages(&results);
    assert_eq!(msgs.len(), 1);
    assert_eq!(msgs[0]["role"], "tool");
    assert_eq!(msgs[0]["tool_call_id"], "c1");
}
