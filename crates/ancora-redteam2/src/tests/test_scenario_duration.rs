use crate::scenario::{RedTeamScenario, ScenarioKind};

#[test]
fn duration_before_complete() {
    let s = RedTeamScenario::new("sc-1", "t1", "N", ScenarioKind::InitialAccess, 10);
    assert_eq!(s.duration(30), 20);
}

#[test]
fn duration_after_complete() {
    let mut s = RedTeamScenario::new("sc-1", "t1", "N", ScenarioKind::InitialAccess, 10);
    s.complete(50);
    assert_eq!(s.duration(999), 40);
}

#[test]
fn duration_saturates() {
    let s = RedTeamScenario::new("sc-1", "t1", "N", ScenarioKind::InitialAccess, 100);
    assert_eq!(s.duration(5), 0);
}
