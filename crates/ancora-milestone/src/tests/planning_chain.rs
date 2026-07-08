use ancora_ageval::PlanningMetric;
use ancora_orchestrate::{fan_out, AgentRole, AgentSpec};
use serde_json::json;

#[test]
fn planning_chain_fan_out_to_spec() {
    let tasks = fan_out(
        "orch-1",
        "sub-agent",
        vec![json!("task1"), json!("task2")],
        "root",
    );
    assert_eq!(tasks.len(), 2);

    let quality = PlanningMetric::score(
        &["task1".into(), "task2".into()],
        &["task1".into(), "task2".into()],
    );
    assert_eq!(quality, 1.0);

    let spec = AgentSpec::new("orch-1", AgentRole::Orchestrator, "planning agent");
    assert_eq!(spec.agent_id, "orch-1");
}
