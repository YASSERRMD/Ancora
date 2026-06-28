use ancora_ageval::{GuardrailMetric, PlanningMetric, RoutingMetric};
use ancora_guard::{GuardrailJournal, GuardrailPolicy, InjectionInputGuardrail, SafetyOutputGuardrail};

#[test]
fn routing_plus_escalation_plus_verifier() {
    // Route: favor low cost + high quality
    let score = RoutingMetric::score(0.9, 20, 100);
    assert!(score > 0.7);

    // Escalate via guardrail: injection blocked at input, unsafe blocked at output
    let mut policy = GuardrailPolicy::new();
    policy.add_input(InjectionInputGuardrail);
    policy.add_output(SafetyOutputGuardrail);
    let mut journal = GuardrailJournal::default();

    let input_ok = policy.check_input("what is the weather?", &mut journal, 1);
    let output_ok = policy.check_output("It is sunny.", &mut journal, 2);

    assert_eq!(input_ok, ancora_guard::GuardrailOutcome::Pass);
    assert_eq!(output_ok, ancora_guard::GuardrailOutcome::Pass);

    // Verify via eval metric: planning quality
    let plan_score = PlanningMetric::score(
        &["route".into(), "check-guardrails".into(), "respond".into()],
        &["route".into(), "check-guardrails".into(), "respond".into()],
    );
    assert_eq!(plan_score, 1.0);
}

#[test]
fn guardrail_catch_rate_combines_with_routing() {
    let route_score = RoutingMetric::score(0.8, 50, 100);
    let guard_score = GuardrailMetric::score(5, 5);
    let combined = (route_score + guard_score) / 2.0;
    assert!(combined > 0.5);
}
