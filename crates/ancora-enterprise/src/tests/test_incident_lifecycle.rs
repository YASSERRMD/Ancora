use crate::incident::{EnterpriseIncident, IncidentSeverity, IncidentStatus};

#[test]
fn investigate() {
    let mut i = EnterpriseIncident::new("i1", "t1", "N", IncidentSeverity::Medium, "d", 1);
    i.investigate();
    assert_eq!(i.status, IncidentStatus::Investigating);
    assert!(!i.is_open());
    assert!(!i.is_resolved());
}

#[test]
fn resolve_sets_tick() {
    let mut i = EnterpriseIncident::new("i1", "t1", "N", IncidentSeverity::High, "d", 5);
    i.resolve(50);
    assert_eq!(i.status, IncidentStatus::Resolved);
    assert_eq!(i.resolved_tick, Some(50));
    assert!(i.is_resolved());
    assert_eq!(i.time_to_resolve(999), 45);
}

#[test]
fn close_incident() {
    let mut i = EnterpriseIncident::new("i1", "t1", "N", IncidentSeverity::Low, "d", 1);
    i.close();
    assert_eq!(i.status, IncidentStatus::Closed);
    assert!(i.is_resolved());
}

#[test]
fn critical_flag() {
    let crit = EnterpriseIncident::new("i1", "t1", "N", IncidentSeverity::Critical, "d", 1);
    let low = EnterpriseIncident::new("i2", "t1", "N", IncidentSeverity::Low, "d", 1);
    assert!(crit.is_critical());
    assert!(!low.is_critical());
}

#[test]
fn time_to_resolve_unresolved() {
    let i = EnterpriseIncident::new("i1", "t1", "N", IncidentSeverity::Low, "d", 10);
    assert_eq!(i.time_to_resolve(30), 20);
}
