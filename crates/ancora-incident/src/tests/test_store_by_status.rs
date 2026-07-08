use crate::incident::{Incident, IncidentStatus, Severity};
use crate::store::IncidentStore;

#[test]
fn by_status_filters() {
    let mut s = IncidentStore::new();
    s.insert(Incident::new("i1", "t1", "A", Severity::Low, 1));
    if let Some(i) = s.get_mut("i1") {
        i.triage();
    }
    s.insert(Incident::new("i2", "t1", "B", Severity::Low, 1));
    assert_eq!(s.by_status(&IncidentStatus::Triaged).len(), 1);
    assert_eq!(s.by_status(&IncidentStatus::Detected).len(), 1);
}
