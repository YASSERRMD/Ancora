//! Integration tests: dashboard JSON is valid and contains expected fields.

use crate::alerting::{Alert, Severity};
use crate::dashboard::{DashboardSnapshot, HealthStatus, Metric, extract_top_level_str};

#[test]
fn dashboard_json_valid_and_contains_timestamp() {
    let ts = "2026-06-28T12:00:00Z";
    let snap = DashboardSnapshot::new(ts, vec![], vec![]);
    let json = snap.to_json();
    assert!(json.contains("\"timestamp\""), "missing timestamp field");
    let extracted = extract_top_level_str(&json, "timestamp");
    assert_eq!(extracted, Some(ts));
}

#[test]
fn dashboard_json_health_ok_with_no_alerts() {
    let snap = DashboardSnapshot::new("ts", vec![], vec![]);
    let json = snap.to_json();
    let health = extract_top_level_str(&json, "health");
    assert_eq!(health, Some("ok"));
    assert_eq!(snap.health, HealthStatus::Ok);
}

#[test]
fn dashboard_json_health_incident_with_critical_alert() {
    let alerts = vec![Alert::new(Severity::Critical, "cost_drift", "cost spiked")];
    let snap = DashboardSnapshot::new("ts", vec![], alerts);
    assert_eq!(snap.health, HealthStatus::Incident);
    let json = snap.to_json();
    assert!(json.contains("\"incident\""));
}

#[test]
fn dashboard_json_includes_metrics() {
    let metrics = vec![
        Metric { name: "mean_cost_micros".into(), value: 150.0, unit: "microdollars".into() },
        Metric { name: "input_drift_z".into(), value: 1.2, unit: "z-score".into() },
    ];
    let snap = DashboardSnapshot::new("ts", metrics, vec![]);
    let json = snap.to_json();
    assert!(json.contains("\"mean_cost_micros\""));
    assert!(json.contains("150"));
    assert!(json.contains("\"input_drift_z\""));
}

#[test]
fn dashboard_json_includes_alert_details() {
    let alerts = vec![Alert::new(Severity::Warning, "tool_drift", "tool usage changed")
        .with_metric(4.5)];
    let snap = DashboardSnapshot::new("ts", vec![], alerts);
    let json = snap.to_json();
    assert!(json.contains("\"WARNING\""));
    assert!(json.contains("\"tool_drift\""));
    assert!(json.contains("4.5"));
}
