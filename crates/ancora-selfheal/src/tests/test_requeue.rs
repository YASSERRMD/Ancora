use crate::requeue::AutoRequeue;

#[test]
fn requeue_enqueues_once() {
    let mut r = AutoRequeue::new(3);
    assert!(r.enqueue("run-1", 0, 10));
    assert_eq!(r.pending_count(), 1);
}

#[test]
fn pop_due_returns_ready_entries() {
    let mut r = AutoRequeue::new(3);
    r.enqueue("run-1", 0, 5);
    let due = r.pop_due(10);
    assert_eq!(due.len(), 1);
    assert_eq!(due[0].run_id, "run-1");
    assert_eq!(r.pending_count(), 0);
}

#[test]
fn not_due_not_returned() {
    let mut r = AutoRequeue::new(3);
    r.enqueue("run-1", 0, 100);
    let due = r.pop_due(10);
    assert!(due.is_empty());
}

#[test]
fn max_attempts_blocks_requeue() {
    let mut r = AutoRequeue::new(2);
    r.enqueue("run-1", 0, 1);
    r.enqueue("run-1", 0, 1);
    assert!(!r.enqueue("run-1", 0, 1));
    assert_eq!(r.attempts_for("run-1"), 2);
}
