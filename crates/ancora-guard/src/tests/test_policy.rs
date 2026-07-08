use crate::allowdeny::AllowDenyGuardrail;
use crate::guardrail::GuardrailOutcome;
use crate::journal::GuardrailJournal;
use crate::pii::PiiInputGuardrail;
use crate::policy::GuardrailPolicy;
use crate::safety::SafetyOutputGuardrail;

#[test]
fn policy_checks_input_guardrails() {
    let mut policy = GuardrailPolicy::new();
    policy.add_input(PiiInputGuardrail);
    let mut journal = GuardrailJournal::default();
    let outcome = policy.check_input("call user@test.com", &mut journal, 1);
    assert!(matches!(outcome, GuardrailOutcome::Repair(_)));
}

#[test]
fn policy_checks_output_guardrails() {
    let mut policy = GuardrailPolicy::new();
    policy.add_output(SafetyOutputGuardrail);
    let mut journal = GuardrailJournal::default();
    let outcome = policy.check_output("<script>xss</script>", &mut journal, 1);
    assert!(matches!(outcome, GuardrailOutcome::Block(_)));
}

#[test]
fn policy_checks_action_guardrails() {
    let mut policy = GuardrailPolicy::new();
    policy.add_action(AllowDenyGuardrail::deny(vec!["rm_rf"]));
    let mut journal = GuardrailJournal::default();
    let outcome = policy.check_action("rm_rf", "{}", &mut journal, 1);
    assert!(matches!(outcome, GuardrailOutcome::Block(_)));
}

#[test]
fn clean_inputs_pass_all_guardrails() {
    let mut policy = GuardrailPolicy::new();
    policy.add_input(PiiInputGuardrail);
    let mut journal = GuardrailJournal::default();
    assert_eq!(
        policy.check_input("summarize this text", &mut journal, 1),
        GuardrailOutcome::Pass
    );
}
