use ancora_guard::{AllowDenyGuardrail, GuardrailJournal, GuardrailOutcome, GuardrailPolicy};
use ancora_toolsynth::{spec_from_goal, ApprovalGate, EffectClass, SandboxRunner, SynthRegistry};

#[test]
fn tool_synthesis_plus_guardrails() {
    let spec = spec_from_goal("search documents");
    assert_eq!(spec.effect_class, EffectClass::ReadOnly);

    let mut registry = SynthRegistry::default();
    registry.register(spec.clone());
    assert_eq!(registry.len(), 1);

    let mut gate = ApprovalGate::default();
    gate.approve(&spec.name);
    assert!(gate.is_approved(&spec.name));

    // Sandbox allows ReadOnly
    let input = serde_json::json!({});
    assert!(SandboxRunner::execute(&spec, &input).is_ok());

    // Guard: only approved tools may be called
    let mut policy = GuardrailPolicy::new();
    policy.add_action(AllowDenyGuardrail::allow_only(vec![spec.name.as_str()]));
    let mut journal = GuardrailJournal::default();
    let outcome = policy.check_action(&spec.name, "{}", &mut journal, 1);
    assert_eq!(outcome, GuardrailOutcome::Pass);
}

#[test]
fn unapproved_tool_blocked_by_guardrail() {
    let mut policy = GuardrailPolicy::new();
    policy.add_action(AllowDenyGuardrail::deny(vec!["drop_table"]));
    let mut journal = GuardrailJournal::default();
    let outcome = policy.check_action("drop_table", "{}", &mut journal, 1);
    assert!(matches!(outcome, GuardrailOutcome::Block(_)));
}
