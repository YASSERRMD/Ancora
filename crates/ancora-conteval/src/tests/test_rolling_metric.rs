use crate::rolling_metric::RollingMetric;

#[test]
fn test_empty_metric_returns_none() {
    let m = RollingMetric::new(5);
    assert!(m.mean().is_none());
    assert!(m.min().is_none());
    assert!(m.max().is_none());
    assert!(m.std_dev().is_none());
}

#[test]
fn test_single_observation() {
    let mut m = RollingMetric::new(5);
    m.push(1, 0.8);
    assert_eq!(m.mean().unwrap(), 0.8);
    assert_eq!(m.min().unwrap(), 0.8);
    assert_eq!(m.max().unwrap(), 0.8);
    assert!(m.std_dev().is_none()); // need >= 2 for std_dev
}

#[test]
fn test_mean_updates_correctly() {
    let mut m = RollingMetric::new(10);
    m.push(1, 0.6);
    m.push(2, 0.8);
    m.push(3, 1.0);
    let mean = m.mean().unwrap();
    let expected = (0.6 + 0.8 + 1.0) / 3.0;
    assert!((mean - expected).abs() < 1e-10);
}

#[test]
fn test_rolling_evicts_oldest() {
    let mut m = RollingMetric::new(3);
    m.push(1, 1.0);
    m.push(2, 2.0);
    m.push(3, 3.0);
    // Now push a 4th - first should be evicted.
    m.push(4, 4.0);
    assert_eq!(m.len(), 3);
    // Oldest entry (1.0) should be gone; mean = (2+3+4)/3
    let expected = (2.0 + 3.0 + 4.0) / 3.0;
    assert!((m.mean().unwrap() - expected).abs() < 1e-10);
}

#[test]
fn test_min_max() {
    let mut m = RollingMetric::new(10);
    m.push(1, 0.5);
    m.push(2, 0.9);
    m.push(3, 0.3);
    assert!((m.min().unwrap() - 0.3).abs() < 1e-10);
    assert!((m.max().unwrap() - 0.9).abs() < 1e-10);
}

#[test]
fn test_std_dev_computed() {
    let mut m = RollingMetric::new(10);
    m.push(1, 2.0);
    m.push(2, 4.0);
    // mean = 3, variance = ((2-3)^2 + (4-3)^2)/2 = 1, std_dev = 1
    let sd = m.std_dev().unwrap();
    assert!((sd - 1.0).abs() < 1e-10);
}

#[test]
fn test_latest_returns_last_pushed() {
    let mut m = RollingMetric::new(5);
    m.push(10, 0.5);
    m.push(20, 0.7);
    let latest = m.latest().unwrap();
    assert_eq!(latest.timestamp, 20);
    assert!((latest.score - 0.7).abs() < 1e-10);
}

#[test]
fn test_observations_slice_length() {
    let mut m = RollingMetric::new(4);
    for i in 0u64..6 {
        m.push(i, i as f64 * 0.1);
    }
    assert_eq!(m.observations().len(), 4);
}
