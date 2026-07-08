use crate::incident::{Incident, Severity};
use crate::query::IncidentQuery;

#[test]
fn active_only_filter() {
    let i1 = Incident::new("i1", "t1", "A", Severity::Low, 1);
    let mut i2 = Incident::new("i2", "t1", "B", Severity::Low, 1);
    i2.resolve(10);
    let all = vec![i1, i2];
    let result = IncidentQuery::new().active_only().run(all.iter());
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].id, "i1");
}

#[test]
fn query_combined_filters() {
    let i1 = Incident::new("i1", "t1", "A", Severity::High, 1);
    let i2 = Incident::new("i2", "t1", "B", Severity::Low, 1);
    let mut i3 = Incident::new("i3", "t1", "C", Severity::High, 1);
    i3.resolve(10);
    let all = vec![i1, i2, i3];
    let result = IncidentQuery::new()
        .severity(Severity::High)
        .active_only()
        .run(all.iter());
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].id, "i1");
}
