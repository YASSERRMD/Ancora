use crate::confidence::ConfidenceAggregator;

#[test]
fn aggregate_computes_mean() {
    let agg = ConfidenceAggregator::new(0.5);
    let mean = agg.aggregate(&[0.4, 0.6, 0.8]);
    let expected = (0.4 + 0.6 + 0.8) / 3.0;
    assert!((mean - expected).abs() < 1e-10);
}

#[test]
fn is_confident_when_mean_above_threshold() {
    let agg = ConfidenceAggregator::new(0.5);
    assert!(agg.is_confident(&[0.6, 0.8]));
}

#[test]
fn not_confident_when_mean_below_threshold() {
    let agg = ConfidenceAggregator::new(0.5);
    assert!(!agg.is_confident(&[0.1, 0.2]));
}

#[test]
fn empty_scores_give_zero_confidence() {
    let agg = ConfidenceAggregator::new(0.5);
    assert_eq!(agg.aggregate(&[]), 0.0);
    assert!(!agg.is_confident(&[]));
}
