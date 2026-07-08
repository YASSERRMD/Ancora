use crate::schema::ToolResult;
use serde_json::{Map, Value};

/// Merges multiple tool results into a single context Value for the next model turn.
pub fn merge_results(results: &[ToolResult]) -> Value {
    let mut map = Map::new();
    for r in results {
        let key = format!("{}_{}", r.tool_name, r.call_id);
        let entry = if r.is_error {
            serde_json::json!({ "error": r.output })
        } else {
            serde_json::json!({ "ok": r.output })
        };
        map.insert(key, entry);
    }
    Value::Object(map)
}

pub fn results_to_messages(results: &[ToolResult]) -> Vec<Value> {
    results
        .iter()
        .map(|r| {
            serde_json::json!({
                "role": "tool",
                "tool_call_id": r.call_id,
                "content": if r.is_error {
                    format!("error: {}", r.output)
                } else {
                    r.output.to_string()
                }
            })
        })
        .collect()
}

pub fn error_count(results: &[ToolResult]) -> usize {
    results.iter().filter(|r| r.is_error).count()
}
