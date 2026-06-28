use crate::outcome::{Observation, OutcomeStore};

#[test]
fn metric_collected_per_variant() {
    let mut store = OutcomeStore::new();
    for i in 0..10 {
        store.record(Observation::new("exp-1", format!("u-{i}"), "control", 0.7));
        store.record(Observation::new("exp-1", format!("u-{}", i + 100), "treatment", 0.85));
    }

    let ctrl_values = store.values_for_variant("exp-1", "control");
    let trt_values = store.values_for_variant("exp-1", "treatment");

    assert_eq!(ctrl_values.len(), 10);
    assert_eq!(trt_values.len(), 10);
    assert!(ctrl_values.iter().all(|&v| (v - 0.7).abs() < 1e-10));
    assert!(trt_values.iter().all(|&v| (v - 0.85).abs() < 1e-10));
}

#[test]
fn stats_mean_is_correct() {
    let mut store = OutcomeStore::new();
    let values = [1.0, 2.0, 3.0, 4.0, 5.0];
    for (i, &v) in values.iter().enumerate() {
        store.record(Observation::new("e", format!("u{i}"), "ctrl", v));
    }
    let stats = store.stats_for_variant("e", "ctrl").unwrap();
    assert!((stats.mean - 3.0).abs() < 1e-10, "mean should be 3.0");
}

#[test]
fn stats_variance_sample_is_correct() {
    let mut store = OutcomeStore::new();
    // Two values: mean=5, sample var = ((5-0)^2 + (5-10)^2) / 1 = 50
    store.record(Observation::new("e", "u0", "v", 0.0));
    store.record(Observation::new("e", "u1", "v", 10.0));
    let stats = store.stats_for_variant("e", "v").unwrap();
    assert!((stats.variance - 50.0).abs() < 1e-10);
}

#[test]
fn no_observations_returns_none() {
    let store = OutcomeStore::new();
    assert!(store.stats_for_variant("missing", "ctrl").is_none());
}
