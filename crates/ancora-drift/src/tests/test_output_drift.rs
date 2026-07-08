//! Integration tests for output drift detection.

use crate::output_drift::OutputDriftDetector;
use crate::reference::{ReferenceBuilder, Stats};

/// Build a reference with genuine output-length variance so std_dev > 0.
fn baseline() -> crate::reference::ReferenceDistribution {
    let mut b = ReferenceBuilder::new();
    b.add("q", "ok", 100, 50, &[], "openai");
    b.add(
        "q",
        "yes that is correct and I agree",
        100,
        50,
        &[],
        "openai",
    );
    b.add("q", "no", 100, 50, &[], "openai");
    b.add(
        "q",
        "certainly! here is a longer reply for you to read",
        100,
        50,
        &[],
        "openai",
    );
    b.build().unwrap()
}

#[test]
fn output_drift_detected_on_very_long_outputs() {
    let reference = baseline();
    // Push current mean to 5000 chars - well beyond reference distribution.
    let current = Stats::from_slice(&[5000.0]).unwrap();
    let detector = OutputDriftDetector::new(2.0);
    let result = detector.check(&reference, &current).unwrap();
    assert!(result.drifted, "expected drift: {result:?}");
}

#[test]
fn output_drift_not_detected_on_stable_outputs() {
    let reference = baseline();
    // Current mean is near the reference mean (~22 chars).
    let current = Stats::from_slice(&[20.0]).unwrap();
    let detector = OutputDriftDetector::new(3.0);
    let result = detector.check(&reference, &current).unwrap();
    assert!(!result.drifted);
}
