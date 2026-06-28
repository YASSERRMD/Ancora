use crate::contradiction::ContradictionDetector;
use crate::decompose::StepDecomposer;

#[test]
fn contradiction_detected_between_steps() {
    let steps = StepDecomposer::decompose(vec![
        "NOT: the earth is round".into(),
        "the earth is round".into(),
    ]);
    let pairs = ContradictionDetector::detect(&steps);
    assert_eq!(pairs, vec![(0, 1)]);
}

#[test]
fn no_contradiction_in_consistent_steps() {
    let steps = StepDecomposer::decompose(vec![
        "the sky is blue".into(),
        "water is wet".into(),
    ]);
    let pairs = ContradictionDetector::detect(&steps);
    assert!(pairs.is_empty());
}

#[test]
fn contradiction_detected_in_reverse_order() {
    let steps = StepDecomposer::decompose(vec![
        "gravity exists".into(),
        "NOT: gravity exists".into(),
    ]);
    let pairs = ContradictionDetector::detect(&steps);
    assert_eq!(pairs, vec![(0, 1)]);
}

#[test]
fn multiple_contradictions_detected() {
    let steps = StepDecomposer::decompose(vec![
        "A is true".into(),
        "NOT: A is true".into(),
        "B is true".into(),
        "NOT: B is true".into(),
    ]);
    let pairs = ContradictionDetector::detect(&steps);
    assert!(pairs.contains(&(0, 1)));
    assert!(pairs.contains(&(2, 3)));
}
