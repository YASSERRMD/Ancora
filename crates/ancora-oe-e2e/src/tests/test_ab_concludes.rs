use crate::ab_e2e::{conclude_experiment, ExperimentConclusion, ExperimentMetrics, Variant};

#[test]
fn ab_experiment_concludes_with_winner() {
    let variants = vec![
        Variant::new("control", "Original prompt"),
        Variant::new("treatment", "Improved prompt"),
    ];

    let mut metrics = ExperimentMetrics::new();
    // Control scores lower.
    for _ in 0..10 {
        metrics.record("control", 0.70);
    }
    // Treatment scores higher.
    for _ in 0..10 {
        metrics.record("treatment", 0.85);
    }

    let conclusion = conclude_experiment(&metrics, &variants, 5);
    assert_eq!(
        conclusion,
        ExperimentConclusion::Winner("treatment".to_string())
    );
}

#[test]
fn ab_experiment_is_inconclusive_without_enough_samples() {
    let variants = vec![Variant::new("control", "A"), Variant::new("treatment", "B")];

    let mut metrics = ExperimentMetrics::new();
    metrics.record("control", 0.8);
    metrics.record("treatment", 0.9);

    let conclusion = conclude_experiment(&metrics, &variants, 10);
    assert_eq!(conclusion, ExperimentConclusion::Inconclusive);
}

#[test]
fn ab_experiment_ties_when_means_are_equal() {
    let variants = vec![Variant::new("v1", "A"), Variant::new("v2", "B")];

    let mut metrics = ExperimentMetrics::new();
    for _ in 0..5 {
        metrics.record("v1", 0.80);
        metrics.record("v2", 0.80);
    }

    let conclusion = conclude_experiment(&metrics, &variants, 5);
    assert_eq!(conclusion, ExperimentConclusion::Tie);
}

#[test]
fn experiment_metrics_count_and_mean_are_correct() {
    let mut metrics = ExperimentMetrics::new();
    for v in [1.0, 2.0, 3.0] {
        metrics.record("v", v);
    }
    assert_eq!(metrics.count("v"), 3);
    assert!((metrics.mean("v").unwrap() - 2.0).abs() < f64::EPSILON);
}
