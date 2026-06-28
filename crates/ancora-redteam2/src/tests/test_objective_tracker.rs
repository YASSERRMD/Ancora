use crate::objective::{ObjectiveTracker, RedTeamObjective};

#[test]
fn empty_tracker() {
    let t = ObjectiveTracker::new();
    assert_eq!(t.count(), 0);
    assert_eq!(t.achieved_count(), 0);
    assert_eq!(t.pending_count(), 0);
    assert!((t.progress() - 0.0).abs() < f64::EPSILON);
}

#[test]
fn add_and_count() {
    let mut t = ObjectiveTracker::new();
    t.add(RedTeamObjective::new("o1", "sc1", "d1"));
    t.add(RedTeamObjective::new("o2", "sc1", "d2"));
    assert_eq!(t.count(), 2);
    assert_eq!(t.pending_count(), 2);
}

#[test]
fn achieve_and_progress() {
    let mut t = ObjectiveTracker::new();
    t.add(RedTeamObjective::new("o1", "sc1", "d1"));
    t.add(RedTeamObjective::new("o2", "sc1", "d2"));
    t.get_mut("o1").unwrap().achieve(10);
    assert_eq!(t.achieved_count(), 1);
    assert!((t.progress() - 0.5).abs() < 1e-9);
}

#[test]
fn for_scenario_filters() {
    let mut t = ObjectiveTracker::new();
    t.add(RedTeamObjective::new("o1", "sc1", "d1"));
    t.add(RedTeamObjective::new("o2", "sc2", "d2"));
    assert_eq!(t.for_scenario("sc1").len(), 1);
    assert_eq!(t.for_scenario("sc2").len(), 1);
}
