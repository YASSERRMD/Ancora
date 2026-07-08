use crate::incident::IncidentSeverity;

#[test]
fn low() {
    assert_eq!(IncidentSeverity::Low.to_string(), "LOW");
}
#[test]
fn medium() {
    assert_eq!(IncidentSeverity::Medium.to_string(), "MEDIUM");
}
#[test]
fn high() {
    assert_eq!(IncidentSeverity::High.to_string(), "HIGH");
}
#[test]
fn critical() {
    assert_eq!(IncidentSeverity::Critical.to_string(), "CRITICAL");
}
