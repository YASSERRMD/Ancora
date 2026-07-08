use crate::runbook::{Runbook, RunbookStep, StepStatus};

#[test]
fn runbook_empty_progress() {
    let rb = Runbook::new("rb1", "Test", "i1");
    assert_eq!(rb.step_count(), 0);
    assert!(!rb.is_complete());
    assert_eq!(rb.progress(), 0.0);
}

#[test]
fn runbook_add_and_complete_steps() {
    let mut rb = Runbook::new("rb1", "Test", "i1");
    rb.add_step(RunbookStep::new("s1", "Step 1", "Do thing 1"));
    rb.add_step(RunbookStep::new("s2", "Step 2", "Do thing 2"));
    assert_eq!(rb.step_count(), 2);
    assert!(!rb.is_complete());
    if let Some(s) = rb.get_step_mut("s1") {
        s.complete(10);
    }
    if let Some(s) = rb.get_step_mut("s2") {
        s.complete(20);
    }
    assert!(rb.is_complete());
    assert_eq!(rb.progress(), 1.0);
}

#[test]
fn runbook_skip_counts_as_done() {
    let mut rb = Runbook::new("rb1", "Test", "i1");
    rb.add_step(RunbookStep::new("s1", "Step 1", "Desc"));
    if let Some(s) = rb.get_step_mut("s1") {
        s.skip();
    }
    assert!(rb.is_complete());
}

#[test]
fn step_fail_is_not_done() {
    let mut s = RunbookStep::new("s1", "Step", "Desc");
    s.fail();
    assert!(!s.is_done());
    assert_eq!(s.status, StepStatus::Failed);
}

#[test]
fn step_start() {
    let mut s = RunbookStep::new("s1", "Step", "Desc");
    s.start();
    assert_eq!(s.status, StepStatus::InProgress);
}

#[test]
fn completed_count() {
    let mut rb = Runbook::new("rb1", "Test", "i1");
    rb.add_step(RunbookStep::new("s1", "A", "Desc"));
    rb.add_step(RunbookStep::new("s2", "B", "Desc"));
    if let Some(s) = rb.get_step_mut("s1") {
        s.complete(1);
    }
    assert_eq!(rb.completed_count(), 1);
    assert_eq!(rb.pending_count(), 1);
}
