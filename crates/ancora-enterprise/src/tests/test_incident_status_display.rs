use crate::incident::IncidentStatus;

#[test]
fn open() {
    assert_eq!(IncidentStatus::Open.to_string(), "OPEN");
}
#[test]
fn investigating() {
    assert_eq!(IncidentStatus::Investigating.to_string(), "INVESTIGATING");
}
#[test]
fn contained() {
    assert_eq!(IncidentStatus::Contained.to_string(), "CONTAINED");
}
#[test]
fn resolved() {
    assert_eq!(IncidentStatus::Resolved.to_string(), "RESOLVED");
}
#[test]
fn closed() {
    assert_eq!(IncidentStatus::Closed.to_string(), "CLOSED");
}
