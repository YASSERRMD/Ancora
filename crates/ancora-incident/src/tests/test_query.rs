use crate::incident::{Incident, IncidentStatus, Severity};
use crate::query::IncidentQuery;

#[test]
fn query_by_severity() {
    let i1 = Incident::new("i1", "t1", "A", Severity::High, 1);
    let i2 = Incident::new("i2", "t1", "B", Severity::Low, 1);
    let all = vec![i1, i2];
    let result = IncidentQuery::new()
        .severity(Severity::High)
        .run(all.iter());
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].id, "i1");
}

#[test]
fn query_by_status() {
    let mut i1 = Incident::new("i1", "t1", "A", Severity::Low, 1);
    i1.triage();
    let i2 = Incident::new("i2", "t1", "B", Severity::Low, 1);
    let all = vec![i1, i2];
    let result = IncidentQuery::new()
        .status(IncidentStatus::Triaged)
        .run(all.iter());
    assert_eq!(result.len(), 1);
}

#[test]
fn query_by_assignee() {
    let mut i1 = Incident::new("i1", "t1", "A", Severity::Low, 1);
    i1.assign("alice");
    let i2 = Incident::new("i2", "t1", "B", Severity::Low, 1);
    let all = vec![i1, i2];
    let result = IncidentQuery::new().assignee("alice").run(all.iter());
    assert_eq!(result.len(), 1);
}

#[test]
fn query_all() {
    let i1 = Incident::new("i1", "t1", "A", Severity::High, 1);
    let i2 = Incident::new("i2", "t1", "B", Severity::Critical, 1);
    let all = vec![i1, i2];
    let result = IncidentQuery::new().run(all.iter());
    assert_eq!(result.len(), 2);
}
