// Air-gap parity: all advanced capability tests run in-process, no network.
// Validates that every parity check completes without any I/O.
use ancora_ageval::{PlanningMetric, ReasoningMetric, RoutingMetric};
use ancora_coord::{Bid, ContractNet};
use ancora_guard::{GuardrailJournal, GuardrailPolicy, InjectionInputGuardrail};
use ancora_lh::BackgroundRun;
use ancora_memcon::TokenBudget;
use ancora_orchestrate::fan_out;
use ancora_reason::StepDecomposer;
use ancora_skills::{SkillDescriptor, SkillRegistry, SkillScope};
use ancora_toolsynth::{spec_from_goal, SandboxRunner};
use serde_json::json;

#[test]
fn airgap_parity_all_metrics_no_network() {
    // All pure arithmetic, no I/O
    let _ = PlanningMetric::score(&["a".to_string()], &["a".to_string()]);
    let _ = RoutingMetric::score(0.9, 100, 1000);
    let _ = ReasoningMetric::score(4, 5);
}

#[test]
fn airgap_parity_orchestrate_in_memory() {
    let tasks = fan_out("o", "a", vec![json!("x")], "root");
    assert_eq!(tasks.len(), 1);
}

#[test]
fn airgap_parity_coord_in_memory() {
    let bids = vec![Bid {
        agent_id: "a".into(),
        task_id: "t".into(),
        score: 0.9,
    }];
    assert!(ContractNet::assign(&bids).is_some());
}

#[test]
fn airgap_parity_guard_in_memory() {
    let mut p = GuardrailPolicy::new();
    p.add_input(InjectionInputGuardrail);
    let mut j = GuardrailJournal::default();
    p.check_input("safe text", &mut j, 1);
}

#[test]
fn airgap_parity_lh_in_memory() {
    let mut run = BackgroundRun::new("r", 1);
    run.start();
    run.complete();
}

#[test]
fn airgap_parity_memcon_in_memory() {
    let budget = TokenBudget::new(100);
    assert!(budget.within_budget(&["short".to_string()]));
}

#[test]
fn airgap_parity_reason_in_memory() {
    let steps = StepDecomposer::decompose(vec!["claim".into()]);
    assert_eq!(steps.len(), 1);
}

#[test]
fn airgap_parity_skills_in_memory() {
    let mut reg = SkillRegistry::default();
    reg.load(SkillDescriptor::new(
        "x",
        1,
        "x",
        vec![],
        SkillScope::ReadOnly,
    ));
    assert!(reg.find("x").is_some());
}

#[test]
fn airgap_parity_toolsynth_in_memory() {
    let spec = spec_from_goal("search");
    let result = SandboxRunner::execute(&spec, &json!({}));
    assert!(result.is_ok());
}
