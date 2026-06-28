use crate::queue::ReviewQueue;

#[test]
fn low_confidence_run_queued() {
    let mut q = ReviewQueue::with_threshold(0.75);
    let added = q.submit("run-low", 0.3);
    assert!(added, "Run with confidence 0.3 should be queued at threshold 0.75");
    assert_eq!(q.pending().len(), 1);
}

#[test]
fn high_confidence_run_not_queued() {
    let mut q = ReviewQueue::with_threshold(0.75);
    let added = q.submit("run-high", 0.95);
    assert!(!added, "Run with confidence 0.95 should not be queued at threshold 0.75");
    assert!(q.pending().is_empty());
}

#[test]
fn boundary_confidence_not_queued() {
    let mut q = ReviewQueue::with_threshold(0.75);
    // Exactly at threshold - not below, so not queued
    let added = q.submit("run-exact", 0.75);
    assert!(!added);
}

#[test]
fn multiple_low_confidence_runs_all_queued() {
    let mut q = ReviewQueue::with_threshold(0.6);
    q.submit("run-1", 0.1);
    q.submit("run-2", 0.4);
    q.submit("run-3", 0.59);
    assert_eq!(q.pending().len(), 3);
    assert_eq!(q.len(), 3);
}

#[test]
fn claiming_removes_from_pending() {
    let mut q = ReviewQueue::with_threshold(0.5);
    q.submit("run-x", 0.2);
    assert_eq!(q.pending().len(), 1);
    let claimed = q.claim("run-x");
    assert!(claimed);
    assert_eq!(q.pending().len(), 0);
    assert_eq!(q.len(), 1); // still in total
}
