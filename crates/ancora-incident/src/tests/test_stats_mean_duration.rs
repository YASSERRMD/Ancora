use crate::incident::{Incident, Severity};
use crate::stats::IncidentStats;

#[test]
fn mean_duration_with_resolved() {
    let mut i1 = Incident::new("i1", "t1", "A", Severity::Low, 0);
    i1.resolve(100);
    let mut i2 = Incident::new("i2", "t1", "B", Severity::Low, 0);
    i2.resolve(200);
    let v: Vec<&Incident> = vec![&i1, &i2];
    let stats = IncidentStats::for_tenant(&v, "t1", 999);
    assert!((stats.mean_duration - 150.0).abs() < f64::EPSILON);
}

#[test]
fn mean_duration_active() {
    let i1 = Incident::new("i1", "t1", "A", Severity::Low, 0);
    let v: Vec<&Incident> = vec![&i1];
    let stats = IncidentStats::for_tenant(&v, "t1", 50);
    assert_eq!(stats.mean_duration, 50.0);
}
