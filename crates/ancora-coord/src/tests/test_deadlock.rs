use crate::deadlock::DeadlockDetector;

#[test]
fn deadlock_detected_in_cycle() {
    let mut det = DeadlockDetector::default();
    det.add_wait("a", "b");
    det.add_wait("b", "a");
    assert!(det.has_deadlock());
}

#[test]
fn no_deadlock_in_dag() {
    let mut det = DeadlockDetector::default();
    det.add_wait("a", "b");
    det.add_wait("b", "c");
    assert!(!det.has_deadlock());
}

#[test]
fn deadlock_broken() {
    let mut det = DeadlockDetector::default();
    det.add_wait("x", "y");
    det.add_wait("y", "x");
    let victim = det.break_cycle().unwrap();
    assert!(!victim.is_empty());
    assert!(!det.has_deadlock());
}
