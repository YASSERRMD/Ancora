use crate::incident::{Incident, Severity};
use crate::postmortem::Postmortem;
use crate::runbook::{Runbook, RunbookStep};
use crate::timeline::IncidentTimeline;

#[test]
fn completion_rate_zero_steps() {
    let i = Incident::new("i1", "t1", "T", Severity::Low, 0);
    let tl = IncidentTimeline::new();
    let pm = Postmortem::generate(&i, None, &tl, 10, "R", "R");
    assert_eq!(pm.runbook_completion_rate(), 0.0);
}

#[test]
fn completion_rate_all_done() {
    let i = Incident::new("i1", "t1", "T", Severity::Low, 0);
    let mut rb = Runbook::new("rb1", "R", "i1");
    rb.add_step(RunbookStep::new("s1", "A", "D"));
    if let Some(s) = rb.get_step_mut("s1") {
        s.complete(1);
    }
    let tl = IncidentTimeline::new();
    let pm = Postmortem::generate(&i, Some(&rb), &tl, 10, "R", "R");
    assert_eq!(pm.runbook_completion_rate(), 1.0);
}
