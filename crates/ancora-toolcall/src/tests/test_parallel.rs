use crate::parallel::ParallelDispatcher;
use crate::schema::{ToolCall, ToolResult};
use serde_json::json;

#[test]
fn parallel_execute_succeeds() {
    let pd = ParallelDispatcher::new(4);
    let calls = vec![
        ToolCall::new("c1", "tool-a", json!({})),
        ToolCall::new("c2", "tool-b", json!({})),
    ];
    let results = pd
        .execute(calls, |c| {
            Ok(ToolResult::ok(&c.call_id, &c.tool_name, json!("ok"), 10))
        })
        .unwrap();
    assert_eq!(results.len(), 2);
}

#[test]
fn parallel_limit_exceeded_errors() {
    let pd = ParallelDispatcher::new(2);
    let calls = vec![
        ToolCall::new("c1", "t", json!({})),
        ToolCall::new("c2", "t", json!({})),
        ToolCall::new("c3", "t", json!({})),
    ];
    let result = pd.execute(calls, |c| {
        Ok(ToolResult::ok(&c.call_id, &c.tool_name, json!("ok"), 0))
    });
    assert!(result.is_err());
}
