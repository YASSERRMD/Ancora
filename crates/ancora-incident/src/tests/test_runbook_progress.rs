use crate::runbook::{Runbook, RunbookStep};

#[test]
fn progress_partial() {
    let mut rb = Runbook::new("rb1", "Test", "i1");
    rb.add_step(RunbookStep::new("s1", "A", "D"));
    rb.add_step(RunbookStep::new("s2", "B", "D"));
    rb.add_step(RunbookStep::new("s3", "C", "D"));
    rb.add_step(RunbookStep::new("s4", "D", "D"));
    if let Some(s) = rb.get_step_mut("s1") {
        s.complete(1);
    }
    if let Some(s) = rb.get_step_mut("s2") {
        s.complete(2);
    }
    let progress = rb.progress();
    assert!((progress - 0.5).abs() < f64::EPSILON);
}

#[test]
fn progress_full() {
    let mut rb = Runbook::new("rb1", "Test", "i1");
    rb.add_step(RunbookStep::new("s1", "A", "D"));
    if let Some(s) = rb.get_step_mut("s1") {
        s.complete(1);
    }
    assert_eq!(rb.progress(), 1.0);
}
