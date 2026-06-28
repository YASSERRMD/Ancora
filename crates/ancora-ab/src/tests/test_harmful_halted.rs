use crate::guardrail::{Guardrail, GuardrailDirection, GuardrailStatus};
use crate::outcome::{Observation, OutcomeStore};

#[test]
fn harmful_variant_triggers_guardrail() {
    let mut store = OutcomeStore::new();
    // Control error rate ~0.05
    for i in 0..10 {
        store.record(Observation::new("exp-g", format!("c{i}"), "control", 0.05));
    }
    // Treatment error rate ~0.25 (5x the control - above 2x threshold)
    for i in 0..10 {
        store.record(Observation::new("exp-g", format!("t{i}"), "treatment", 0.25));
    }

    let guardrail = Guardrail::new(
        "error-rate-guard",
        "exp-g",
        "error_rate",
        "control",
        GuardrailDirection::TreatmentAbove,
        2.0, // trigger if treatment > 2x control
    );

    let status = guardrail.evaluate(&store, &["treatment"]);
    assert!(
        matches!(status, GuardrailStatus::Triggered { .. }),
        "expected Triggered, got {:?}",
        status
    );
}

#[test]
fn safe_variant_does_not_trigger() {
    let mut store = OutcomeStore::new();
    for i in 0..10 {
        store.record(Observation::new("exp-safe", format!("c{i}"), "control", 0.10));
    }
    for i in 0..10 {
        store.record(Observation::new("exp-safe", format!("t{i}"), "treatment", 0.12));
    }

    let guardrail = Guardrail::new(
        "error-guard",
        "exp-safe",
        "error_rate",
        "control",
        GuardrailDirection::TreatmentAbove,
        2.0,
    );

    let status = guardrail.evaluate(&store, &["treatment"]);
    assert_eq!(status, GuardrailStatus::Clear);
}

#[test]
fn absolute_floor_guardrail_triggers() {
    let mut store = OutcomeStore::new();
    for i in 0..5 {
        store.record(Observation::new("exp-floor", format!("t{i}"), "treatment", 0.3));
    }

    let guardrail = Guardrail::new(
        "quality-floor",
        "exp-floor",
        "quality_score",
        "control",
        GuardrailDirection::AbsoluteFloor { floor: 0.5 },
        1.0,
    );

    let status = guardrail.evaluate(&store, &["treatment"]);
    assert!(
        matches!(status, GuardrailStatus::Triggered { .. }),
        "expected Triggered for floor check"
    );
}

#[test]
fn insufficient_data_returns_insufficient() {
    let store = OutcomeStore::new();
    let guardrail = Guardrail::new(
        "g",
        "exp-no-data",
        "err",
        "control",
        GuardrailDirection::TreatmentAbove,
        2.0,
    );
    let status = guardrail.evaluate(&store, &["treatment"]);
    assert_eq!(status, GuardrailStatus::Insufficient);
}
