use crate::alert::{AlertStatus, AlertStore, ThreatAlert};
use crate::indicator::ThreatLevel;

#[test]
fn alert_suppress() {
    let mut store = AlertStore::new();
    store.add(ThreatAlert::new("a1", "t1", "i1", ThreatLevel::High, "M", 1));
    if let Some(a) = store.get_mut("a1") { a.suppress(); }
    assert_eq!(store.open().len(), 0);
    assert_eq!(store.get_mut("a1").map(|a| a.status.clone()), Some(AlertStatus::Suppressed));
}

#[test]
fn alert_store_count() {
    let mut store = AlertStore::new();
    for i in 0..3 {
        store.add(ThreatAlert::new(format!("a{}", i), "t1", "ix", ThreatLevel::Medium, "M", i));
    }
    assert_eq!(store.count(), 3);
}
