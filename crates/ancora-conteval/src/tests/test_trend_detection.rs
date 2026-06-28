use crate::trend::{analyse_trend, TrendDetector, TrendDirection};

#[test]
fn test_analyse_trend_needs_at_least_two_points() {
    assert!(analyse_trend(&[], 0.01, 0.01).is_none());
    assert!(analyse_trend(&[1.0], 0.01, 0.01).is_none());
}

#[test]
fn test_flat_series_is_stable() {
    let scores = vec![0.8, 0.8, 0.8, 0.8, 0.8];
    let result = analyse_trend(&scores, 0.01, 0.01).unwrap();
    assert_eq!(result.direction, TrendDirection::Stable);
    assert!(result.slope.abs() < 1e-10);
}

#[test]
fn test_declining_series_is_degrading() {
    // Consistently decreasing scores.
    let scores = vec![0.9, 0.8, 0.7, 0.6, 0.5];
    let result = analyse_trend(&scores, 0.05, 0.05).unwrap();
    assert_eq!(result.direction, TrendDirection::Degrading);
    assert!(result.slope < 0.0);
}

#[test]
fn test_improving_series_is_improving() {
    let scores = vec![0.5, 0.6, 0.7, 0.8, 0.9];
    let result = analyse_trend(&scores, 0.05, 0.05).unwrap();
    assert_eq!(result.direction, TrendDirection::Improving);
    assert!(result.slope > 0.0);
}

#[test]
fn test_trend_detector_pushes_and_analyses() {
    let mut det = TrendDetector::new("model-x", 5, 0.05, 0.05);
    det.push(0.9);
    let result = det.push(0.8);
    // With 2 points we get a result.
    assert!(result.is_some());
    assert_eq!(det.history_len(), 2);
}

#[test]
fn test_trend_detector_degrades() {
    let mut det = TrendDetector::new("m", 10, 0.05, 0.05);
    for &s in &[0.9f64, 0.85, 0.8, 0.75, 0.7] {
        det.push(s);
    }
    let result = det.push(0.65).unwrap();
    assert_eq!(result.direction, TrendDirection::Degrading);
}

#[test]
fn test_trend_window_limits_regression_points() {
    let mut det = TrendDetector::new("m", 3, 0.01, 0.01);
    // Push 10 values; only last 3 used.
    for i in 0..10 {
        det.push(i as f64 * 0.1);
    }
    let result = det.push(0.99).unwrap();
    // Slope should reflect only the recent trend.
    assert_eq!(result.window, 3);
}

#[test]
fn test_analyse_trend_window_size_reported() {
    let scores = vec![0.5, 0.6, 0.7];
    let r = analyse_trend(&scores, 0.01, 0.01).unwrap();
    assert_eq!(r.window, 3);
}
