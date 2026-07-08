use crate::scenario::{RedTeamScenario, ScenarioKind, ScenarioStatus};

#[test]
fn basic_fields() {
    let s = RedTeamScenario::new(
        "sc-1",
        "t1",
        "Test Scenario",
        ScenarioKind::LateralMovement,
        10,
    );
    assert_eq!(s.id, "sc-1");
    assert_eq!(s.tenant_id, "t1");
    assert_eq!(s.name, "Test Scenario");
    assert_eq!(s.kind, ScenarioKind::LateralMovement);
    assert_eq!(s.status, ScenarioStatus::Pending);
    assert_eq!(s.created_tick, 10);
    assert!(s.mitre_tactic.is_none());
    assert!(s.completed_tick.is_none());
}

#[test]
fn with_mitre() {
    let s = RedTeamScenario::new("sc-2", "t1", "Name", ScenarioKind::DefenseEvasion, 5)
        .with_mitre("TA0005");
    assert_eq!(s.mitre_tactic.as_deref(), Some("TA0005"));
}

#[test]
fn with_metadata() {
    let s = RedTeamScenario::new("sc-3", "t1", "Name", ScenarioKind::InitialAccess, 1)
        .with_metadata("severity", "critical");
    assert_eq!(
        s.metadata.get("severity").map(|v| v.as_str()),
        Some("critical")
    );
}
