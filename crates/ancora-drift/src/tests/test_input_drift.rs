//! Integration tests for input drift detection.

use crate::input_drift::InputDriftDetector;
use crate::reference::{ReferenceBuilder, Stats};

/// Build a reference with genuine input-length variance so std_dev > 0.
fn baseline() -> crate::reference::ReferenceDistribution {
    let mut b = ReferenceBuilder::new();
    // Mix short and long inputs so std_dev is non-zero.
    b.add("hi", "ok", 100, 50, &[], "openai");
    b.add("hello world", "ok", 100, 50, &[], "openai");
    b.add("hey", "ok", 100, 50, &[], "openai");
    b.add("how are you doing today?", "ok", 100, 50, &[], "openai");
    b.build().unwrap()
}

#[test]
fn input_drift_detected_when_inputs_get_much_longer() {
    let reference = baseline();
    // Reference mean ~10 chars; push current mean to 500 chars.
    let current = Stats::from_slice(&[500.0, 510.0, 490.0]).unwrap();
    let detector = InputDriftDetector::new(2.0);
    let result = detector.check(&reference, &current).unwrap();
    assert!(result.drifted, "expected drift but got none: {result:?}");
    assert!(result.mean_z_score > 2.0);
}

#[test]
fn input_drift_not_detected_on_stable_inputs() {
    let reference = baseline();
    // Current mean is close to the reference mean (~10 chars).
    let current = Stats::from_slice(&[10.0]).unwrap();
    let detector = InputDriftDetector::new(3.0);
    let result = detector.check(&reference, &current).unwrap();
    assert!(!result.drifted);
}
