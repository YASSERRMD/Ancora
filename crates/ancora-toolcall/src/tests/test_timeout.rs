use crate::timeout::ToolTimeoutTracker;

#[test]
fn not_timed_out_before_deadline() {
    let mut t = ToolTimeoutTracker::new();
    t.register("c1", "search", 0, 5000); // deadline at 5s
    assert!(!t.is_timed_out("c1", 4));
}

#[test]
fn timed_out_at_deadline() {
    let mut t = ToolTimeoutTracker::new();
    t.register("c1", "search", 0, 5000);
    assert!(t.is_timed_out("c1", 5));
}

#[test]
fn timed_out_calls_list() {
    let mut t = ToolTimeoutTracker::new();
    t.register("c1", "search", 0, 2000); // deadline 2s
    t.register("c2", "exec", 0, 10000);  // deadline 10s
    let expired = t.timed_out_calls(3);
    assert_eq!(expired.len(), 1);
    assert_eq!(expired[0].0, "c1");
}
