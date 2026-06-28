use crate::runbook::{Runbook, RunbookStep};

#[test]
fn not_complete_with_pending() {
    let mut rb = Runbook::new("rb1", "Test", "i1");
    rb.add_step(RunbookStep::new("s1", "A", "D"));
    rb.add_step(RunbookStep::new("s2", "B", "D"));
    if let Some(s) = rb.get_step_mut("s1") { s.complete(1); }
    assert!(!rb.is_complete());
}

#[test]
fn complete_all_skipped() {
    let mut rb = Runbook::new("rb1", "Test", "i1");
    rb.add_step(RunbookStep::new("s1", "A", "D"));
    rb.add_step(RunbookStep::new("s2", "B", "D"));
    if let Some(s) = rb.get_step_mut("s1") { s.skip(); }
    if let Some(s) = rb.get_step_mut("s2") { s.skip(); }
    assert!(rb.is_complete());
}
