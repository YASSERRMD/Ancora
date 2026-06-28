use crate::incident::{Incident, Severity};
use crate::stats::IncidentStats;

#[test]
fn stats_empty() {
    let stats = IncidentStats::for_tenant(&[], "t1", 100);
    assert_eq!(stats.total, 0);
    assert_eq!(stats.mean_duration, 0.0);
}

#[test]
fn stats_for_tenant() {
    let i1 = Incident::new("i1", "t1", "A", Severity::High, 0);
    let i2 = Incident::new("i2", "t1", "B", Severity::Critical, 10);
    let i3 = Incident::new("i3", "t2", "C", Severity::Low, 0);
    let v: Vec<&Incident> = vec![&i1, &i2, &i3];
    let stats = IncidentStats::for_tenant(&v, "t1", 100);
    assert_eq!(stats.total, 2);
    assert_eq!(stats.active, 2);
}

#[test]
fn stats_by_severity() {
    let i1 = Incident::new("i1", "t1", "A", Severity::High, 0);
    let i2 = Incident::new("i2", "t1", "B", Severity::High, 0);
    let v: Vec<&Incident> = vec![&i1, &i2];
    let stats = IncidentStats::for_tenant(&v, "t1", 100);
    assert_eq!(stats.by_severity.get("HIGH").copied(), Some(2));
}

#[test]
fn stats_resolved_count() {
    let mut i1 = Incident::new("i1", "t1", "A", Severity::Low, 0);
    i1.resolve(50);
    let v: Vec<&Incident> = vec![&i1];
    let stats = IncidentStats::for_tenant(&v, "t1", 100);
    assert_eq!(stats.resolved, 1);
    assert_eq!(stats.active, 0);
}
