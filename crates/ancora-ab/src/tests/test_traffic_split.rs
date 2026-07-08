use crate::assignment::assign;
use crate::experiment::{Experiment, Metric, MetricKind, Variant};

/// Run many assignments and verify that variant proportions are close to the declared weights.
#[test]
fn traffic_split_proportions_hold() {
    let experiment = Experiment::new(
        "split-test",
        "Test traffic split",
        vec![Variant::new("control", 0.5), Variant::new("treatment", 0.5)],
        Metric::new("success_rate", MetricKind::Maximize),
    )
    .unwrap();

    let n = 10_000;
    let mut control_count = 0usize;
    let mut treatment_count = 0usize;

    for i in 0..n {
        let key = format!("user-{i}");
        let assignment = assign(&experiment, &key);
        match assignment.variant_name.as_str() {
            "control" => control_count += 1,
            "treatment" => treatment_count += 1,
            other => panic!("unexpected variant: {other}"),
        }
    }

    let control_ratio = control_count as f64 / n as f64;
    let treatment_ratio = treatment_count as f64 / n as f64;

    // Allow 5% absolute deviation from the declared 50/50 split.
    assert!(
        (control_ratio - 0.5).abs() < 0.05,
        "control ratio {control_ratio} is too far from 0.5"
    );
    assert!(
        (treatment_ratio - 0.5).abs() < 0.05,
        "treatment ratio {treatment_ratio} is too far from 0.5"
    );
}

#[test]
fn unequal_traffic_split_proportions_hold() {
    let experiment = Experiment::new(
        "split-unequal",
        "Unequal split",
        vec![Variant::new("control", 0.8), Variant::new("treatment", 0.2)],
        Metric::new("latency_ms", MetricKind::Minimize),
    )
    .unwrap();

    let n = 10_000;
    let mut counts = std::collections::HashMap::new();
    for i in 0..n {
        let key = format!("uid-{i}");
        let a = assign(&experiment, &key);
        *counts.entry(a.variant_name).or_insert(0usize) += 1;
    }

    let control_ratio = counts["control"] as f64 / n as f64;
    let treatment_ratio = counts["treatment"] as f64 / n as f64;

    assert!(
        (control_ratio - 0.8).abs() < 0.05,
        "control ratio {control_ratio:.3} should be ~0.8"
    );
    assert!(
        (treatment_ratio - 0.2).abs() < 0.05,
        "treatment ratio {treatment_ratio:.3} should be ~0.2"
    );
}
