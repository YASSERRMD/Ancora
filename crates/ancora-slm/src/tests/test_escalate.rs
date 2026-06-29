use crate::escalate::{can_escalate_to, run_with_escalation, EscalationPolicy};
use crate::model::{ModelSpec, ModelTier};
use crate::verifier::{ValidJsonVerifier, Verifier};

#[test]
fn test_escalation_not_triggered_when_slm_succeeds() {
    let policy = EscalationPolicy { max_slm_attempts: 2, escalation_tier: ModelTier::Large };
    let verifiers: Vec<Box<dyn Verifier>> = vec![Box::new(ValidJsonVerifier)];

    let slm_fn = |_: &str| r#"{"answer": "ok"}"#.to_string();
    let llm_fn = |_: &str| r#"{"answer": "escalated"}"#.to_string();

    let result = run_with_escalation(
        "Give JSON",
        &policy,
        &verifiers,
        slm_fn,
        llm_fn,
        "slm-model",
        "llm-model",
    );

    assert!(!result.escalated, "should not escalate when SLM succeeds");
    assert_eq!(result.model_id, "slm-model");
}

#[test]
fn test_escalation_triggers_on_failure() {
    let policy = EscalationPolicy { max_slm_attempts: 2, escalation_tier: ModelTier::Large };
    let verifiers: Vec<Box<dyn Verifier>> = vec![Box::new(ValidJsonVerifier)];

    // SLM always returns prose (invalid JSON).
    let slm_fn = |_: &str| "I am unable to provide JSON output.".to_string();
    let llm_fn = |_: &str| r#"{"answer": "from-llm"}"#.to_string();

    let result = run_with_escalation(
        "Give JSON",
        &policy,
        &verifiers,
        slm_fn,
        llm_fn,
        "slm-model",
        "llm-model",
    );

    assert!(result.escalated, "should escalate when SLM repeatedly fails");
    assert_eq!(result.model_id, "llm-model");
    assert!(result.escalation_reason.is_some());
}

#[test]
fn test_escalation_result_contains_llm_output() {
    let policy = EscalationPolicy::default();
    let verifiers: Vec<Box<dyn Verifier>> = vec![Box::new(ValidJsonVerifier)];

    let slm_fn = |_: &str| "bad output".to_string();
    let llm_fn = |_: &str| r#"{"escalated": true}"#.to_string();

    let result = run_with_escalation(
        "Task",
        &policy,
        &verifiers,
        slm_fn,
        llm_fn,
        "slm",
        "llm",
    );

    assert!(
        result.text.contains("escalated"),
        "result text should come from LLM"
    );
}

#[test]
fn test_can_escalate_to_large_model() {
    let policy = EscalationPolicy { escalation_tier: ModelTier::Large, max_slm_attempts: 1 };
    let large = ModelSpec::large("gpt-4");
    let small = ModelSpec::small("phi-3-mini");
    assert!(can_escalate_to(&large, &policy));
    assert!(!can_escalate_to(&small, &policy));
}

#[test]
fn test_escalation_records_attempt_count() {
    let policy = EscalationPolicy { max_slm_attempts: 3, escalation_tier: ModelTier::Large };
    let verifiers: Vec<Box<dyn Verifier>> = vec![Box::new(ValidJsonVerifier)];

    let slm_fn = |_: &str| "not json".to_string();
    let llm_fn = |_: &str| r#"{"ok": 1}"#.to_string();

    let result = run_with_escalation("Q", &policy, &verifiers, slm_fn, llm_fn, "s", "l");
    assert_eq!(
        result.slm_attempts, 3,
        "should record max_slm_attempts as attempt count"
    );
}
