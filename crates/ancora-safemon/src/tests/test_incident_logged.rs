use crate::incident_log::{IncidentLog, IncidentSeverity};

#[test]
fn incident_is_logged_and_retrievable() {
    let mut log = IncidentLog::new(100);
    let id = log.log(
        IncidentSeverity::High,
        "pii",
        "Email address found in output",
        "user@example.com",
        Some("agent-xyz".to_string()),
    );
    let incident = log.find_by_id(id).expect("incident should be present");
    assert_eq!(incident.severity, IncidentSeverity::High);
    assert_eq!(incident.category, "pii");
    assert_eq!(incident.agent_id, Some("agent-xyz".to_string()));
}

#[test]
fn multiple_incidents_accumulate() {
    let mut log = IncidentLog::new(100);
    log.log(IncidentSeverity::Low, "cat1", "d", "e", None);
    log.log(IncidentSeverity::Medium, "cat2", "d", "e", None);
    log.log(IncidentSeverity::High, "cat1", "d", "e", None);
    assert_eq!(log.count(), 3);
}

#[test]
fn filter_by_severity_works() {
    let mut log = IncidentLog::new(100);
    log.log(IncidentSeverity::High, "pii", "d", "e", None);
    log.log(IncidentSeverity::Medium, "toxicity", "d", "e", None);
    log.log(IncidentSeverity::High, "policy", "d", "e", None);

    let high = log.by_severity(&IncidentSeverity::High);
    assert_eq!(high.len(), 2);
}

#[test]
fn incident_json_contains_required_fields() {
    let mut log = IncidentLog::new(100);
    log.log(
        IncidentSeverity::Critical,
        "policy",
        "Breach detected",
        "secret data",
        None,
    );
    let json = log.to_json();
    assert!(json.contains("CRITICAL"));
    assert!(json.contains("policy"));
}

#[test]
fn incident_timestamps_are_monotonic() {
    let mut log = IncidentLog::new(100);
    log.log(IncidentSeverity::Info, "cat", "d", "e", None);
    log.log(IncidentSeverity::Info, "cat", "d", "e", None);

    let incidents: Vec<_> = log.all().collect();
    assert!(incidents[1].timestamp > incidents[0].timestamp);
}
