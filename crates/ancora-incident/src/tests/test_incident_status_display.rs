use crate::incident::IncidentStatus;

#[test]
fn status_display() {
    assert_eq!(format!("{}", IncidentStatus::Detected), "DETECTED");
    assert_eq!(format!("{}", IncidentStatus::Triaged), "TRIAGED");
    assert_eq!(format!("{}", IncidentStatus::Investigating), "INVESTIGATING");
    assert_eq!(format!("{}", IncidentStatus::Mitigating), "MITIGATING");
    assert_eq!(format!("{}", IncidentStatus::Resolved), "RESOLVED");
    assert_eq!(format!("{}", IncidentStatus::Closed), "CLOSED");
}
