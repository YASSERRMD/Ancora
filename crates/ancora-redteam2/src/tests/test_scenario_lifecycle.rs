use crate::scenario::{RedTeamScenario, ScenarioKind, ScenarioStatus};

#[test]
fn start_changes_status() {
    let mut s = RedTeamScenario::new("sc-1", "t1", "N", ScenarioKind::LateralMovement, 1);
    s.start();
    assert_eq!(s.status, ScenarioStatus::Running);
    assert!(s.is_active());
}

#[test]
fn complete_sets_tick() {
    let mut s = RedTeamScenario::new("sc-1", "t1", "N", ScenarioKind::LateralMovement, 1);
    s.start();
    s.complete(100);
    assert_eq!(s.status, ScenarioStatus::Completed);
    assert_eq!(s.completed_tick, Some(100));
    assert!(s.is_done());
    assert!(!s.is_active());
}

#[test]
fn fail_marks_failed() {
    let mut s = RedTeamScenario::new("sc-1", "t1", "N", ScenarioKind::LateralMovement, 1);
    s.fail();
    assert_eq!(s.status, ScenarioStatus::Failed);
    assert!(s.is_done());
}

#[test]
fn abort_marks_aborted() {
    let mut s = RedTeamScenario::new("sc-1", "t1", "N", ScenarioKind::LateralMovement, 1);
    s.abort();
    assert_eq!(s.status, ScenarioStatus::Aborted);
    assert!(s.is_done());
}
