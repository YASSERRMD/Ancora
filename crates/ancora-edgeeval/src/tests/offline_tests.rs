//! Offline eval mode tests.

use crate::offline::{MockScorer, OfflineConfig, OfflineDataset, OfflineEvalRunner, OfflineSample};

#[test]
fn test_mock_scorer_exact_match() {
    let scorer = MockScorer::new(42);
    assert!((scorer.score("Paris", "Paris") - 1.0).abs() < 1e-9);
}

#[test]
fn test_mock_scorer_mismatch_partial() {
    let scorer = MockScorer::new(42);
    let score = scorer.score("London", "Paris");
    assert!((0.0..=1.0).contains(&score));
}

#[test]
fn test_offline_dataset_add_retrieve() {
    let mut ds = OfflineDataset::new();
    ds.add(OfflineSample::new("x1", "input", "output"));
    assert_eq!(ds.len(), 1);
    assert_eq!(ds.samples()[0].id, "x1");
}

#[test]
fn test_offline_runner_produces_all_ids() {
    let mut ds = OfflineDataset::new();
    ds.add(OfflineSample::new("a", "in_a", "out_a"));
    ds.add(OfflineSample::new("b", "in_b", "out_b"));
    let config = OfflineConfig::new();
    let runner = OfflineEvalRunner::new(config);
    let outputs = [("a", "out_a"), ("b", "wrong")];
    let results = runner.run(&ds, &outputs);
    assert_eq!(results.len(), 2);
    assert!((results[0].1 - 1.0).abs() < 1e-9, "a should score 1.0");
}
