use crate::incident::{Incident, Severity};
use crate::summary::IncidentSummary;

#[test]
fn summary_healthy() {
    let i1 = Incident::new("i1", "t1", "A", Severity::Low, 1);
    let mut i2 = Incident::new("i2", "t1", "B", Severity::Medium, 1);
    i2.assign("alice");
    let v: Vec<&Incident> = vec![&i1, &i2];
    let s = IncidentSummary::generate(&v, "t1");
    assert_eq!(s.total, 2);
    assert_eq!(s.critical_count, 0);
}

#[test]
fn summary_unhealthy_critical() {
    let i1 = Incident::new("i1", "t1", "A", Severity::Critical, 1);
    let v: Vec<&Incident> = vec![&i1];
    let s = IncidentSummary::generate(&v, "t1");
    assert!(!s.is_healthy());
    assert_eq!(s.critical_count, 1);
}

#[test]
fn summary_active_count() {
    let i1 = Incident::new("i1", "t1", "A", Severity::Low, 1);
    let mut i2 = Incident::new("i2", "t1", "B", Severity::Low, 1);
    i2.resolve(10);
    let v: Vec<&Incident> = vec![&i1, &i2];
    let s = IncidentSummary::generate(&v, "t1");
    assert_eq!(s.active_count, 1);
    assert_eq!(s.resolved_count, 1);
}

#[test]
fn summary_unassigned() {
    let i1 = Incident::new("i1", "t1", "A", Severity::Medium, 1);
    let v: Vec<&Incident> = vec![&i1];
    let s = IncidentSummary::generate(&v, "t1");
    assert_eq!(s.unassigned_count, 1);
    assert!(!s.is_healthy());
}
