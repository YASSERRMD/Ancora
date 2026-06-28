use ancora_preset::{assemble, government_compliant, research_assistant};
use ancora_ageval::PlanningMetric;

#[test]
fn research_example_runs_without_panic() {
    let spec = assemble(&research_assistant()).expect("research");
    assert_eq!(spec.agent_id, "research-assistant");
}

#[test]
fn government_example_runs_without_panic() {
    let spec = assemble(&government_compliant("us-gov-east-1")).expect("government");
    assert!(spec.system_prompt.contains("air_gap:required"));
}

#[test]
fn planning_example_runs() {
    let steps = vec!["step-1".to_string(), "step-2".to_string()];
    let q = PlanningMetric::score(&steps, &steps);
    assert_eq!(q, 1.0);
}
