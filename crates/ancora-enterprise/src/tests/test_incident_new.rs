use crate::incident::{EnterpriseIncident, IncidentSeverity, IncidentStatus};

#[test]
fn basic_fields() {
    let i = EnterpriseIncident::new(
        "inc-1",
        "t1",
        "SSH brute force",
        IncidentSeverity::High,
        "pentest",
        5,
    );
    assert_eq!(i.id, "inc-1");
    assert_eq!(i.tenant_id, "t1");
    assert_eq!(i.title, "SSH brute force");
    assert_eq!(i.severity, IncidentSeverity::High);
    assert_eq!(i.status, IncidentStatus::Open);
    assert_eq!(i.opened_tick, 5);
    assert!(i.resolved_tick.is_none());
    assert!(i.assignee.is_none());
}

#[test]
fn with_assignee() {
    let i = EnterpriseIncident::new("i1", "t1", "N", IncidentSeverity::Low, "d", 1)
        .with_assignee("alice");
    assert_eq!(i.assignee.as_deref(), Some("alice"));
}
