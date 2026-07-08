use crate::incident::{Incident, IncidentStatus, Severity};
use crate::store::IncidentStore;

#[test]
fn store_insert_get() {
    let mut s = IncidentStore::new();
    let i = Incident::new("i1", "t1", "Test", Severity::High, 1);
    s.insert(i);
    assert!(s.get("i1").is_some());
    assert_eq!(s.count(), 1);
}

#[test]
fn store_remove() {
    let mut s = IncidentStore::new();
    s.insert(Incident::new("i1", "t1", "Test", Severity::Low, 1));
    let removed = s.remove("i1");
    assert!(removed.is_some());
    assert_eq!(s.count(), 0);
}

#[test]
fn store_for_tenant() {
    let mut s = IncidentStore::new();
    s.insert(Incident::new("i1", "t1", "T1", Severity::Low, 1));
    s.insert(Incident::new("i2", "t2", "T2", Severity::Low, 1));
    s.insert(Incident::new("i3", "t1", "T3", Severity::Low, 1));
    assert_eq!(s.for_tenant("t1").len(), 2);
    assert_eq!(s.for_tenant("t2").len(), 1);
}

#[test]
fn store_active() {
    let mut s = IncidentStore::new();
    let i1 = Incident::new("i1", "t1", "T1", Severity::Low, 1);
    let mut i2 = Incident::new("i2", "t1", "T2", Severity::Low, 1);
    i2.resolve(10);
    s.insert(i1);
    s.insert(i2);
    assert_eq!(s.active().len(), 1);
}

#[test]
fn store_get_mut() {
    let mut s = IncidentStore::new();
    s.insert(Incident::new("i1", "t1", "Test", Severity::Low, 1));
    if let Some(i) = s.get_mut("i1") {
        i.triage();
    }
    assert_eq!(
        s.get("i1").map(|i| i.status.clone()),
        Some(IncidentStatus::Triaged)
    );
}
