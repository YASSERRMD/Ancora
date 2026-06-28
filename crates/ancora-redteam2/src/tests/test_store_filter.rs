use crate::scenario::{RedTeamScenario, ScenarioKind, ScenarioStatus};
use crate::store::ScenarioStore;

fn sc(id: &str, tenant_id: &str, kind: ScenarioKind) -> RedTeamScenario {
    RedTeamScenario::new(id, tenant_id, "N", kind, 1)
}

#[test]
fn for_tenant() {
    let mut store = ScenarioStore::new();
    store.insert(sc("sc1", "t1", ScenarioKind::LateralMovement));
    store.insert(sc("sc2", "t2", ScenarioKind::LateralMovement));
    store.insert(sc("sc3", "t1", ScenarioKind::DataExfiltration));
    assert_eq!(store.for_tenant("t1").len(), 2);
    assert_eq!(store.for_tenant("t2").len(), 1);
}

#[test]
fn active_filter() {
    let mut store = ScenarioStore::new();
    let mut s = sc("sc1", "t1", ScenarioKind::LateralMovement);
    s.start();
    store.insert(s);
    store.insert(sc("sc2", "t1", ScenarioKind::InitialAccess));
    assert_eq!(store.active().len(), 1);
}

#[test]
fn by_kind() {
    let mut store = ScenarioStore::new();
    store.insert(sc("sc1", "t1", ScenarioKind::DataExfiltration));
    store.insert(sc("sc2", "t1", ScenarioKind::LateralMovement));
    assert_eq!(store.by_kind(&ScenarioKind::DataExfiltration).len(), 1);
    assert_eq!(store.by_kind(&ScenarioKind::PrivilegeEscalation).len(), 0);
}

#[test]
fn by_status() {
    let mut store = ScenarioStore::new();
    let mut s = sc("sc1", "t1", ScenarioKind::LateralMovement);
    s.complete(100);
    store.insert(s);
    store.insert(sc("sc2", "t1", ScenarioKind::LateralMovement));
    assert_eq!(store.by_status(&ScenarioStatus::Completed).len(), 1);
    assert_eq!(store.by_status(&ScenarioStatus::Pending).len(), 1);
}
