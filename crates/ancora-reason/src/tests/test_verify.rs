use crate::decompose::{StepDecomposer, StepStatus};
use crate::verify::StepVerifier;

#[test]
fn verification_catches_wrong_step() {
    let mut steps = StepDecomposer::decompose(vec!["bad claim".into()]);
    let result = StepVerifier::verify(&mut steps[0], |_| false);
    assert!(!result.passed);
    assert_eq!(steps[0].status, StepStatus::Refuted);
}

#[test]
fn verification_passes_correct_step() {
    let mut steps = StepDecomposer::decompose(vec!["good claim".into()]);
    let result = StepVerifier::verify(&mut steps[0], |_| true);
    assert!(result.passed);
    assert_eq!(steps[0].status, StepStatus::Verified);
}

#[test]
fn verification_result_has_correct_step_index() {
    let mut steps = StepDecomposer::decompose(vec!["a".into(), "b".into()]);
    let result = StepVerifier::verify(&mut steps[1], |_| true);
    assert_eq!(result.step_index, 1);
}

#[test]
fn verification_reason_reflects_outcome() {
    let mut steps = StepDecomposer::decompose(vec!["claim".into()]);
    let r1 = StepVerifier::verify(&mut steps[0], |_| true);
    assert_eq!(r1.reason, "verified");

    let mut steps2 = StepDecomposer::decompose(vec!["claim".into()]);
    let r2 = StepVerifier::verify(&mut steps2[0], |_| false);
    assert_eq!(r2.reason, "refuted by checker");
}
