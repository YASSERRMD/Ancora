use crate::abstain::AbstentionPolicy;
use crate::decompose::{StepDecomposer, StepStatus};

#[test]
fn abstains_on_weak_evidence() {
    let policy = AbstentionPolicy::new(0.5);
    let mut steps = StepDecomposer::decompose(vec!["uncertain claim".into()]);
    let abstained = policy.apply(&mut steps[0], &[0.1, 0.2]);
    assert!(abstained);
    assert_eq!(steps[0].status, StepStatus::Abstained);
}

#[test]
fn does_not_abstain_on_strong_evidence() {
    let policy = AbstentionPolicy::new(0.5);
    let mut steps = StepDecomposer::decompose(vec!["confident claim".into()]);
    let abstained = policy.apply(&mut steps[0], &[0.8, 0.9]);
    assert!(!abstained);
    assert_eq!(steps[0].status, StepStatus::Pending);
}

#[test]
fn abstains_at_exact_threshold_boundary() {
    let policy = AbstentionPolicy::new(0.5);
    let mut steps = StepDecomposer::decompose(vec!["borderline".into()]);
    // mean = 0.5, which equals threshold -> confident
    let abstained = policy.apply(&mut steps[0], &[0.5]);
    assert!(!abstained);
}

#[test]
fn abstains_with_empty_scores() {
    let policy = AbstentionPolicy::new(0.5);
    let mut steps = StepDecomposer::decompose(vec!["no-evidence claim".into()]);
    let abstained = policy.apply(&mut steps[0], &[]);
    assert!(abstained);
    assert_eq!(steps[0].status, StepStatus::Abstained);
}
