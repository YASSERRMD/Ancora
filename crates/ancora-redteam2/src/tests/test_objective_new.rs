use crate::objective::{ObjectiveStatus, RedTeamObjective};

#[test]
fn basic_fields() {
    let obj = RedTeamObjective::new("o1", "sc1", "Gain access");
    assert_eq!(obj.id, "o1");
    assert_eq!(obj.scenario_id, "sc1");
    assert_eq!(obj.description, "Gain access");
    assert_eq!(obj.status, ObjectiveStatus::Pending);
    assert!(obj.achieved_tick.is_none());
}
