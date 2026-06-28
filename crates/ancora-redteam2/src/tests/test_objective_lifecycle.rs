use crate::objective::{ObjectiveStatus, RedTeamObjective};

#[test]
fn start() {
    let mut obj = RedTeamObjective::new("o1", "sc1", "desc");
    obj.start();
    assert_eq!(obj.status, ObjectiveStatus::InProgress);
    assert!(!obj.is_achieved());
    assert!(!obj.is_done());
}

#[test]
fn achieve() {
    let mut obj = RedTeamObjective::new("o1", "sc1", "desc");
    obj.achieve(50);
    assert!(obj.is_achieved());
    assert_eq!(obj.achieved_tick, Some(50));
    assert!(obj.is_done());
}

#[test]
fn fail_objective() {
    let mut obj = RedTeamObjective::new("o1", "sc1", "desc");
    obj.fail();
    assert_eq!(obj.status, ObjectiveStatus::Failed);
    assert!(!obj.is_achieved());
    assert!(obj.is_done());
}
