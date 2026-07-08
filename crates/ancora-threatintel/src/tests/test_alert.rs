use crate::alert::{AlertStatus, AlertStore, ThreatAlert};
use crate::indicator::ThreatLevel;

#[test]
fn alert_new_is_open() {
    let a = ThreatAlert::new("a1", "t1", "i1", ThreatLevel::High, "Bad IP seen", 100);
    assert!(a.is_open());
    assert_eq!(a.status, AlertStatus::Open);
}

#[test]
fn alert_acknowledge() {
    let mut a = ThreatAlert::new(
        "a1",
        "t1",
        "i1",
        ThreatLevel::Medium,
        "Suspicious domain",
        1,
    );
    a.acknowledge();
    assert_eq!(a.status, AlertStatus::Acknowledged);
    assert!(!a.is_open());
}

#[test]
fn alert_store_open() {
    let mut store = AlertStore::new();
    store.add(ThreatAlert::new(
        "a1",
        "t1",
        "i1",
        ThreatLevel::High,
        "M",
        1,
    ));
    store.add(ThreatAlert::new("a2", "t1", "i2", ThreatLevel::Low, "M", 2));
    if let Some(a) = store.get_mut("a2") {
        a.close();
    }
    assert_eq!(store.open().len(), 1);
}

#[test]
fn alert_store_for_tenant() {
    let mut store = AlertStore::new();
    store.add(ThreatAlert::new(
        "a1",
        "t1",
        "i1",
        ThreatLevel::High,
        "M",
        1,
    ));
    store.add(ThreatAlert::new(
        "a2",
        "t2",
        "i2",
        ThreatLevel::High,
        "M",
        2,
    ));
    assert_eq!(store.for_tenant("t1").len(), 1);
}
