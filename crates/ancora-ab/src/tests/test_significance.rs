use crate::analysis::welch_t_test;
use crate::outcome::{Observation, OutcomeStore};

fn make_store_with_two_variants(
    exp_id: &str,
    ctrl_values: &[f64],
    trt_values: &[f64],
) -> OutcomeStore {
    let mut store = OutcomeStore::new();
    for (i, &v) in ctrl_values.iter().enumerate() {
        store.record(Observation::new(exp_id, format!("c{i}"), "control", v));
    }
    for (i, &v) in trt_values.iter().enumerate() {
        store.record(Observation::new(exp_id, format!("t{i}"), "treatment", v));
    }
    store
}

#[test]
fn clearly_significant_difference_detected() {
    // control ~0.5, treatment ~0.9 - large effect with tight variance
    let ctrl: Vec<f64> = (0..50).map(|i| 0.5 + (i % 3) as f64 * 0.01).collect();
    let trt: Vec<f64> = (0..50).map(|i| 0.9 + (i % 3) as f64 * 0.01).collect();

    let store = make_store_with_two_variants("sig-test", &ctrl, &trt);
    let ctrl_stats = store.stats_for_variant("sig-test", "control").unwrap();
    let trt_stats = store.stats_for_variant("sig-test", "treatment").unwrap();

    let result = welch_t_test(&ctrl_stats, &trt_stats, 0.05).unwrap();
    assert!(
        result.is_significant,
        "expected significant result, got p={:.4}",
        result.p_value
    );
    assert!(result.mean_difference > 0.0);
}

#[test]
fn identical_distributions_not_significant() {
    let values: Vec<f64> = (0..30).map(|i| i as f64).collect();
    let store = make_store_with_two_variants("same-test", &values, &values);
    let ctrl_stats = store.stats_for_variant("same-test", "control").unwrap();
    let trt_stats = store.stats_for_variant("same-test", "treatment").unwrap();

    let result = welch_t_test(&ctrl_stats, &trt_stats, 0.05).unwrap();
    assert!(
        !result.is_significant,
        "identical distributions should not be significant, got p={:.4}",
        result.p_value
    );
}

#[test]
fn insufficient_data_returns_error() {
    use crate::analysis::AnalysisError;
    use crate::outcome::VariantStats;

    let one_obs = VariantStats {
        variant_name: "ctrl".to_string(),
        n: 1,
        mean: 0.5,
        variance: 0.0,
    };
    let good = VariantStats {
        variant_name: "trt".to_string(),
        n: 10,
        mean: 0.8,
        variance: 0.01,
    };
    let err = welch_t_test(&one_obs, &good, 0.05).unwrap_err();
    assert_eq!(
        err,
        AnalysisError::InsufficientData {
            variant: "ctrl".to_string(),
            n: 1
        }
    );
}
