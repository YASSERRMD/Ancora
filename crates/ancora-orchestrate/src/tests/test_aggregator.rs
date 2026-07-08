use crate::result_aggregator::{AgentResult, ResultAggregator};
use serde_json::json;

fn result(task_id: &str, success: bool) -> AgentResult {
    AgentResult {
        task_id: task_id.to_string(),
        agent_id: "a".to_string(),
        output: json!("ok"),
        success,
    }
}

#[test]
fn counts_correct() {
    let mut agg = ResultAggregator::new();
    agg.record(result("t1", true));
    agg.record(result("t2", false));
    assert_eq!(agg.successful_count(), 1);
    assert_eq!(agg.failed_count(), 1);
}

#[test]
fn merge_outputs_has_correct_keys() {
    let mut agg = ResultAggregator::new();
    agg.record(result("t1", true));
    agg.record(result("t2", true));
    let merged = agg.merge_outputs(&["t1", "t2"]);
    assert!(merged.get("t1").is_some());
    assert!(merged.get("t2").is_some());
}
