use crate::incident::{Incident, Severity};
use crate::summary::IncidentSummary;

#[test]
fn healthy_no_critical_no_unassigned() {
    let mut i1 = Incident::new("i1", "t1", "A", Severity::Low, 1);
    i1.assign("alice");
    let v: Vec<&Incident> = vec![&i1];
    let s = IncidentSummary::generate(&v, "t1");
    assert!(s.is_healthy());
}

#[test]
fn unhealthy_unassigned() {
    let i1 = Incident::new("i1", "t1", "A", Severity::Medium, 1);
    let v: Vec<&Incident> = vec![&i1];
    let s = IncidentSummary::generate(&v, "t1");
    assert!(!s.is_healthy());
}

#[test]
fn empty_tenant_is_healthy() {
    let s = IncidentSummary::generate(&[], "t1");
    assert_eq!(s.total, 0);
    assert!(s.is_healthy());
}
