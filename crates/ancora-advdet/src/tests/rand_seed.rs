// Random-seed reproducibility: all Ancora advanced capabilities are deterministic
// and contain no use of thread_rng or any non-seeded random source.
// This test file confirms that repeated execution with identical inputs
// produces identical outputs without any seeding mechanism.

use ancora_ageval::{CoordinationMetric, PlanningMetric};
use ancora_orchestrate::fan_out;
use serde_json::json;

#[test]
fn rand_seed_metrics_no_rng() {
    // All metric functions are pure computations; running 5x should give identical results
    let results: Vec<f64> = (0..5)
        .map(|_| PlanningMetric::score(&["a".to_string(), "b".to_string()], &["a".to_string()]))
        .collect();
    assert!(results
        .windows(2)
        .all(|w| (w[0] - w[1]).abs() < f64::EPSILON));
}

#[test]
fn rand_seed_fan_out_no_rng() {
    let inputs = vec![json!("x"), json!("y"), json!("z")];
    let runs: Vec<Vec<String>> = (0..5)
        .map(|_| {
            fan_out("o", "a", inputs.clone(), "root")
                .into_iter()
                .map(|t| t.task_id)
                .collect()
        })
        .collect();
    assert!(runs.windows(2).all(|w| w[0] == w[1]));
}

#[test]
fn rand_seed_coordination_no_rng() {
    let scores: Vec<f64> = (0..5).map(|_| CoordinationMetric::score(4, 3)).collect();
    assert!(scores
        .windows(2)
        .all(|w| (w[0] - w[1]).abs() < f64::EPSILON));
}
