use crate::drift_parity::{
    MetricWindow, detect_drift, DriftSeverity, stable_drift_report, check_drift_parity,
};

#[test]
fn test_metric_window_mean() {
    let mut window = MetricWindow::new("score", 5);
    for v in &[1.0, 2.0, 3.0, 4.0, 5.0] {
        window.push(*v);
    }
    assert_eq!(window.mean(), Some(3.0));
}

#[test]
fn test_metric_window_evicts_old_values() {
    let mut window = MetricWindow::new("score", 3);
    for v in &[1.0, 2.0, 3.0, 4.0] {
        window.push(*v);
    }
    assert_eq!(window.len(), 3);
    assert_eq!(window.mean(), Some(3.0)); // (2+3+4)/3
}

#[test]
fn test_no_drift_within_threshold() {
    let result = detect_drift("metric", "rust", 0.90, 0.88, 0.05);
    assert_eq!(result.severity, DriftSeverity::None);
}

#[test]
fn test_warning_drift() {
    let result = detect_drift("metric", "rust", 1.0, 0.75, 0.05);
    // z = (1.0 - 0.75)/0.05 = 5.0 -> critical
    assert_eq!(result.severity, DriftSeverity::Critical);
}

#[test]
fn test_stable_drift_report_no_critical() {
    let report = stable_drift_report("rust");
    assert!(!report.has_critical(), "stable report should have no critical drift");
}

#[test]
fn test_drift_parity_across_languages() {
    let langs = &["rust", "python", "typescript", "go", "java", "csharp"];
    let reports: Vec<_> = langs
        .iter()
        .map(|&l| (l, stable_drift_report(l)))
        .collect();
    let refs: Vec<(&str, _)> = reports.iter().map(|(l, r)| (*l, r.clone())).collect();
    let issues = check_drift_parity(&refs);
    assert!(issues.is_empty(), "drift parity issues: {:?}", issues);
}
