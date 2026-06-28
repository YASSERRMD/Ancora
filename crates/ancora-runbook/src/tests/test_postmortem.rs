use crate::postmortem::PostMortem;

#[test]
fn new_postmortem_has_no_actions() {
    let pm = PostMortem::new("INC-001", "cache miss");
    assert_eq!(pm.open_actions(), 0);
}

#[test]
fn add_action_increments_open() {
    let mut pm = PostMortem::new("INC-001", "cache miss");
    pm.add_action("Add cache warm-up", "alice", 86400);
    pm.add_action("Add alert for cache hit rate", "bob", 172800);
    assert_eq!(pm.open_actions(), 2);
}

#[test]
fn completed_action_not_counted() {
    let mut pm = PostMortem::new("INC-001", "cause");
    pm.add_action("task", "owner", 1000);
    pm.action_items[0].completed = true;
    assert_eq!(pm.open_actions(), 0);
}

#[test]
fn timeline_events_ordered_by_insertion() {
    let mut pm = PostMortem::new("INC-001", "cause");
    pm.add_event(100, "alert fired");
    pm.add_event(200, "ack");
    assert_eq!(pm.timeline.len(), 2);
    assert_eq!(pm.timeline[0].at_secs, 100);
}
