use ancora_ageval::ReasoningMetric;
use ancora_reason::{
    ContradictionDetector, ReasoningJournal, StepDecomposer, StepStatus, StepVerifier,
};

const EPS: f64 = 1e-9;

#[test]
fn reasoning_parity_score_canonical() {
    assert!((ReasoningMetric::score(4, 5) - 0.8).abs() < EPS);
    assert!((ReasoningMetric::score(5, 5) - 1.0).abs() < EPS);
    assert!((ReasoningMetric::score(0, 5) - 0.0).abs() < EPS);
}

#[test]
fn reasoning_parity_contradiction_detection() {
    let mut steps = StepDecomposer::decompose(vec![
        "gravity exists".into(),
        "NOT: gravity exists".into(),
    ]);
    let pairs = ContradictionDetector::detect(&steps);
    assert_eq!(pairs.len(), 1);
    assert_eq!(pairs[0], (0, 1));
}

#[test]
fn reasoning_parity_verify_step() {
    let mut steps = StepDecomposer::decompose(vec!["water boils at 100C".into()]);
    let result = StepVerifier::verify(&mut steps[0], |_| true);
    assert!(result.passed);
    assert_eq!(steps[0].status, StepStatus::Verified);
}

#[test]
fn reasoning_parity_refute_step() {
    let mut steps = StepDecomposer::decompose(vec!["incorrect claim".into()]);
    let result = StepVerifier::verify(&mut steps[0], |_| false);
    assert!(!result.passed);
    assert_eq!(steps[0].status, StepStatus::Refuted);
}

#[test]
fn reasoning_parity_journal_event_order() {
    let mut j = ReasoningJournal::default();
    j.record(1, ancora_reason::ReasoningEvent::StepVerified { index: 0 });
    j.record(2, ancora_reason::ReasoningEvent::StepVerified { index: 1 });
    j.record(3, ancora_reason::ReasoningEvent::StepRefuted { index: 2 });
    let events = j.replay();
    assert_eq!(events.len(), 3);
}
