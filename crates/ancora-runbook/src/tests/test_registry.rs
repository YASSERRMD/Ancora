use crate::incident::{Incident, Severity};
use crate::incident_registry::IncidentRegistry;

#[test]
fn open_count_correct() {
    let mut reg = IncidentRegistry::new();
    reg.open(Incident::new("INC-1", "t", Severity::P1, 0, "alice"));
    reg.open(Incident::new("INC-2", "t", Severity::P2, 0, "bob"));
    assert_eq!(reg.open_count(), 2);
}

#[test]
fn resolved_incident_not_counted_as_open() {
    let mut reg = IncidentRegistry::new();
    let mut i = Incident::new("INC-1", "t", Severity::P1, 0, "alice");
    i.resolve(100);
    reg.open(i);
    assert_eq!(reg.open_count(), 0);
}

#[test]
fn by_severity_filters_correctly() {
    let mut reg = IncidentRegistry::new();
    reg.open(Incident::new("INC-1", "t", Severity::P1, 0, "a"));
    reg.open(Incident::new("INC-2", "t", Severity::P2, 0, "b"));
    reg.open(Incident::new("INC-3", "t", Severity::P1, 0, "c"));
    assert_eq!(reg.by_severity(&Severity::P1).len(), 2);
    assert_eq!(reg.by_severity(&Severity::P2).len(), 1);
}
