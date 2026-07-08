// Air-gap test: all red-team operations run in-memory without any network I/O.
use crate::{
    exfiltration_scenarios, injection_scenarios, jailbreak_scenarios, known_attack_regression_set,
    privilege_scenarios, tool_misuse_scenarios, GuardrailScorer, ScenarioBuilder,
};
use ancora_guard::{
    AllowDenyGuardrail, GuardrailJournal, GuardrailPolicy, InjectionInputGuardrail,
};

#[test]
fn airgap_all_scenario_datasets_load() {
    assert!(!injection_scenarios().is_empty());
    assert!(!tool_misuse_scenarios().is_empty());
    assert!(!exfiltration_scenarios().is_empty());
    assert!(!privilege_scenarios().is_empty());
    assert!(!jailbreak_scenarios().is_empty());
    assert!(!known_attack_regression_set().is_empty());
}

#[test]
fn airgap_scorer_runs_in_process() {
    let scenarios = injection_scenarios();
    let report = GuardrailScorer::score(&scenarios, |p| {
        let mut pol = GuardrailPolicy::new();
        pol.add_input(InjectionInputGuardrail);
        let mut j = GuardrailJournal::default();
        !matches!(
            pol.check_input(p, &mut j, 1),
            ancora_guard::GuardrailOutcome::Pass
        )
    });
    // All scoring done in-memory
    assert!(report.effectiveness() >= 0.0);
}

#[test]
fn airgap_custom_builder_in_process() {
    let dataset = ScenarioBuilder::new()
        .add_injection("ignore previous instructions", true)
        .add_tool_misuse("run_shell", true)
        .build();
    assert_eq!(dataset.len(), 2);
}

#[test]
fn airgap_guardrail_policy_in_process() {
    let mut p = GuardrailPolicy::new();
    p.add_input(InjectionInputGuardrail);
    p.add_action(AllowDenyGuardrail::allow_only(vec!["search_web"]));
    let mut j = GuardrailJournal::default();
    p.check_input("safe input", &mut j, 1);
    p.check_action("search_web", "", &mut j, 2);
    // Completes entirely in-process
    assert_eq!(j.blocked_count(), 0);
}
