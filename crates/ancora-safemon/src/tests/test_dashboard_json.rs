use crate::dashboard::Dashboard;
use crate::incident_log::{IncidentLog, IncidentSeverity};

#[test]
fn dashboard_json_contains_required_keys() {
    let log = IncidentLog::new(100);
    let dash = Dashboard::new();
    let snap = dash.snapshot(&log);
    let json = snap.to_json();

    assert!(json.contains("total_incidents"));
    assert!(json.contains("severity_counts"));
    assert!(json.contains("top_categories"));
    assert!(json.contains("alerts_fired"));
    assert!(json.contains("is_healthy"));
}

#[test]
fn severity_counts_json_has_all_levels() {
    let log = IncidentLog::new(100);
    let dash = Dashboard::new();
    let snap = dash.snapshot(&log);
    let json = snap.severity_counts.to_json();

    assert!(json.contains("info"));
    assert!(json.contains("low"));
    assert!(json.contains("medium"));
    assert!(json.contains("high"));
    assert!(json.contains("critical"));
}

#[test]
fn dashboard_reflects_incident_counts() {
    let mut log = IncidentLog::new(100);
    log.log(IncidentSeverity::High, "pii", "d", "e", None);
    log.log(IncidentSeverity::Medium, "toxic", "d", "e", None);
    log.log(IncidentSeverity::Critical, "policy", "d", "e", None);

    let dash = Dashboard::new();
    let snap = dash.snapshot(&log);

    assert_eq!(snap.total_incidents, 3);
    assert_eq!(snap.severity_counts.high, 1);
    assert_eq!(snap.severity_counts.medium, 1);
    assert_eq!(snap.severity_counts.critical, 1);
    assert!(!snap.is_healthy);
}

#[test]
fn top_categories_sorted_by_count() {
    let mut log = IncidentLog::new(100);
    for _ in 0..3 {
        log.log(IncidentSeverity::Low, "pii", "d", "e", None);
    }
    for _ in 0..1 {
        log.log(IncidentSeverity::Low, "toxic", "d", "e", None);
    }

    let dash = Dashboard::new();
    let snap = dash.snapshot(&log);
    assert!(!snap.top_categories.is_empty());
    assert_eq!(snap.top_categories[0].0, "pii");
    assert_eq!(snap.top_categories[0].1, 3);
}

#[test]
fn alerts_fired_reflected_in_json() {
    let log = IncidentLog::new(100);
    let mut dash = Dashboard::new();
    dash.set_alerts_fired(7);
    let snap = dash.snapshot(&log);
    let json = snap.to_json();
    assert!(json.contains("\"alerts_fired\":7"));
}
