use crate::scenario::{RedTeamScenario, ScenarioKind};
use crate::store::ScenarioStore;

fn sc(id: &str, tenant_id: &str) -> RedTeamScenario {
    RedTeamScenario::new(id, tenant_id, "N", ScenarioKind::LateralMovement, 1)
}

#[test]
fn empty_store() {
    let store = ScenarioStore::new();
    assert_eq!(store.count(), 0);
}

#[test]
fn insert_and_get() {
    let mut store = ScenarioStore::new();
    store.insert(sc("sc1", "t1"));
    assert_eq!(store.count(), 1);
    assert!(store.get("sc1").is_some());
    assert!(store.get("none").is_none());
}

#[test]
fn get_mut() {
    let mut store = ScenarioStore::new();
    store.insert(sc("sc1", "t1"));
    store.get_mut("sc1").unwrap().start();
    assert!(store.get("sc1").unwrap().is_active());
}
