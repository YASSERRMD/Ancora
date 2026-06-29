//! SLM reliability tests.

use crate::reliability::{ConsistencyChecker, ReliabilityResult, SlmReliabilityEval};

#[test]
fn test_reliability_single_scenario() {
    let mut eval = SlmReliabilityEval::new();
    eval.add_result(ReliabilityResult::new("only", 0.95, 0.9));
    assert!((eval.overall_score() - 0.95).abs() < 1e-9);
    assert!((eval.pass_rate() - 1.0).abs() < 1e-9);
}

#[test]
fn test_consistency_one_item() {
    let score = ConsistencyChecker::score(&["hello"]);
    assert!((score - 1.0).abs() < 1e-9);
}

#[test]
fn test_reliability_with_notes() {
    let r = ReliabilityResult::new("s", 0.8, 0.7).with_notes("edge case triggered");
    assert!(r.notes.is_some());
    assert!(r.passed);
}
