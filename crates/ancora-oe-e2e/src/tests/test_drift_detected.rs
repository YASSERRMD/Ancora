use crate::drift_e2e::{detect_drift, CategoryDriftDetector, DistributionStats, DriftStatus};
use std::collections::HashMap;

#[test]
fn drift_detected_on_shifted_input() {
    // Baseline: values centered at 10 with std_dev ~1.
    let baseline_samples = [9.0, 10.0, 10.0, 11.0, 10.0];
    let baseline = DistributionStats::from_samples(&baseline_samples).unwrap();

    // Candidate mean is far from baseline.
    let status = detect_drift(&baseline, 15.0, 2.0);

    assert!(
        matches!(status, DriftStatus::DriftDetected { .. }),
        "significant shift must be detected as drift"
    );
}

#[test]
fn no_drift_when_distributions_are_similar() {
    let samples = [9.8, 10.0, 10.2, 9.9, 10.1];
    let baseline = DistributionStats::from_samples(&samples).unwrap();

    let status = detect_drift(&baseline, 10.05, 2.0);

    assert_eq!(status, DriftStatus::NoDrift);
}

#[test]
fn distribution_stats_computed_correctly() {
    let samples = [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
    let stats = DistributionStats::from_samples(&samples).unwrap();
    assert!((stats.mean - 5.0).abs() < 1e-9, "mean must be 5.0");
    assert_eq!(stats.count, 8);
}

#[test]
fn empty_samples_returns_none() {
    let result = DistributionStats::from_samples(&[]);
    assert!(result.is_none());
}

#[test]
fn category_drift_detector_flags_shifted_frequency() {
    let mut baseline = HashMap::new();
    baseline.insert("A".to_string(), 0.6);
    baseline.insert("B".to_string(), 0.4);

    let detector = CategoryDriftDetector::new(baseline, 0.1);

    let mut candidate = HashMap::new();
    candidate.insert("A".to_string(), 0.2);
    candidate.insert("B".to_string(), 0.8);

    assert!(
        detector.is_drifted(&candidate),
        "large frequency shift must be detected"
    );
    let shifted = detector.shifted_categories(&candidate);
    assert!(shifted.contains(&"A".to_string()));
    assert!(shifted.contains(&"B".to_string()));
}
