use crate::anomaly::AnomalyDetector;

#[test]
fn spike_detected_as_anomaly() {
    let mut d = AnomalyDetector::new(2.0);
    // Normal baseline around 1.0
    for v in [1.0f64, 1.05, 0.98, 1.02, 0.99, 1.01, 1.03, 0.97] {
        d.add_observation(v);
    }
    let alert = d.check(99999, 10.0);
    assert!(alert.is_some(), "spike of 10.0 should be detected");
    assert!(alert.unwrap().z_score > 2.0, "z-score should exceed threshold 2.0");
}

#[test]
fn normal_value_not_anomalous() {
    let mut d = AnomalyDetector::new(2.0);
    for v in [1.0f64, 1.05, 0.98, 1.02, 0.99, 1.01] {
        d.add_observation(v);
    }
    let alert = d.check(99999, 1.00);
    assert!(alert.is_none(), "1.00 should not be flagged as anomaly");
}

#[test]
fn observe_adds_to_history() {
    let mut d = AnomalyDetector::new(3.0);
    d.add_observation(1.0);
    d.add_observation(1.0);
    d.add_observation(1.0);
    d.observe(1, 1.5);
    assert_eq!(d.history().len(), 4);
}

#[test]
fn below_min_history_returns_none() {
    let d = AnomalyDetector::new(2.0);
    // Only 1 observation - stddev requires >= 2
    let mut d2 = AnomalyDetector::new(2.0);
    d2.add_observation(1.0);
    let alert = d2.check(1, 999.0);
    assert!(alert.is_none(), "need at least 2 observations for stddev");
    let _ = d; // suppress unused warning
}
