use crate::tracker::MigrationTracker;

#[test]
fn current_version_zero_initially() {
    let t = MigrationTracker::new();
    assert_eq!(t.current_version(), 0);
}

#[test]
fn mark_applied_increments_version() {
    let mut t = MigrationTracker::new();
    t.mark_applied(1, 100);
    t.mark_applied(2, 200);
    assert_eq!(t.current_version(), 2);
    assert_eq!(t.applied_count(), 2);
}

#[test]
fn rollback_removes_from_applied() {
    let mut t = MigrationTracker::new();
    t.mark_applied(1, 100);
    t.mark_applied(2, 200);
    t.mark_rolled_back(2, 300);
    assert_eq!(t.current_version(), 1);
    assert_eq!(t.applied_count(), 1);
}

#[test]
fn is_applied_correct() {
    let mut t = MigrationTracker::new();
    t.mark_applied(1, 0);
    assert!(t.is_applied(1));
    assert!(!t.is_applied(2));
}
