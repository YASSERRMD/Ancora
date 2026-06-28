use crate::attack::{AttackOutcome, AttackVector};
use crate::builder::{AttackStepBuilder, ObjectiveBuilder, ScenarioBuilder};
use crate::scenario::{ScenarioKind, ScenarioStatus};

#[test]
fn scenario_builder_basic() {
    let s = ScenarioBuilder::new("sc1", "t1", "Test", ScenarioKind::DefenseEvasion, 10).build();
    assert_eq!(s.id, "sc1");
    assert_eq!(s.tenant_id, "t1");
    assert_eq!(s.kind, ScenarioKind::DefenseEvasion);
    assert_eq!(s.status, ScenarioStatus::Pending);
    assert_eq!(s.created_tick, 10);
    assert!(s.mitre_tactic.is_none());
}

#[test]
fn scenario_builder_with_mitre() {
    let s = ScenarioBuilder::new("sc1", "t1", "Test", ScenarioKind::LateralMovement, 1)
        .mitre("TA0008")
        .build();
    assert_eq!(s.mitre_tactic.as_deref(), Some("TA0008"));
}

#[test]
fn attack_step_builder() {
    let step = AttackStepBuilder::new("s1", "sc1", "Scan", AttackVector::Network, AttackOutcome::Success, 5)
        .technique("T1046")
        .detail("Found open port 22")
        .build();
    assert_eq!(step.id, "s1");
    assert_eq!(step.technique, "T1046");
    assert_eq!(step.detail, "Found open port 22");
    assert!(step.is_successful());
}

#[test]
fn objective_builder() {
    let obj = ObjectiveBuilder::new("o1", "sc1", "Gain root").build();
    assert_eq!(obj.id, "o1");
    assert_eq!(obj.scenario_id, "sc1");
    assert_eq!(obj.description, "Gain root");
}
