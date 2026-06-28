use crate::incident::{Incident, Severity};
use crate::store::IncidentStore;

#[test]
fn by_severity_filters() {
    let mut s = IncidentStore::new();
    s.insert(Incident::new("i1", "t1", "A", Severity::High, 1));
    s.insert(Incident::new("i2", "t1", "B", Severity::Critical, 1));
    s.insert(Incident::new("i3", "t1", "C", Severity::High, 1));
    assert_eq!(s.by_severity(&Severity::High).len(), 2);
    assert_eq!(s.by_severity(&Severity::Critical).len(), 1);
    assert_eq!(s.by_severity(&Severity::Low).len(), 0);
}
