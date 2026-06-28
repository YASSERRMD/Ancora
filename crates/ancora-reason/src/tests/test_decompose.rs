use crate::decompose::{StepDecomposer, StepStatus};

#[test]
fn decomposition_produces_ordered_steps() {
    let claims = vec!["A".into(), "B".into(), "C".into()];
    let steps = StepDecomposer::decompose(claims);
    assert_eq!(steps.len(), 3);
    assert_eq!(steps[0].index, 0);
    assert_eq!(steps[1].index, 1);
    assert_eq!(steps[2].index, 2);
    assert_eq!(steps[0].claim, "A");
}

#[test]
fn all_steps_start_pending() {
    let steps = StepDecomposer::decompose(vec!["X".into(), "Y".into()]);
    for step in &steps {
        assert_eq!(step.status, StepStatus::Pending);
    }
}

#[test]
fn empty_claims_gives_empty_steps() {
    let steps = StepDecomposer::decompose(vec![]);
    assert!(steps.is_empty());
}

#[test]
fn single_claim_decomposed() {
    let steps = StepDecomposer::decompose(vec!["only claim".into()]);
    assert_eq!(steps.len(), 1);
    assert_eq!(steps[0].index, 0);
    assert_eq!(steps[0].claim, "only claim");
}
