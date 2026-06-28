use crate::incident::{Incident, Severity};
use crate::runbook::{Runbook, RunbookStep};
use crate::timeline::IncidentTimeline;
use crate::audit::IncidentAuditLog;
use crate::escalation::{EscalationChannel, EscalationRecord};
use crate::report::IncidentReport;

#[test]
fn report_no_runbook() {
    let i = Incident::new("i1", "t1", "Test", Severity::High, 0);
    let tl = IncidentTimeline::new();
    let audit = IncidentAuditLog::new();
    let report = IncidentReport::generate(&i, None, &tl, &audit, &[], 100);
    assert_eq!(report.runbook_steps_total, 0);
    assert_eq!(report.runbook_progress(), 0.0);
}

#[test]
fn report_with_runbook() {
    let i = Incident::new("i1", "t1", "Test", Severity::Critical, 0);
    let mut rb = Runbook::new("rb1", "Runbook", "i1");
    rb.add_step(RunbookStep::new("s1", "A", "D"));
    rb.add_step(RunbookStep::new("s2", "B", "D"));
    if let Some(s) = rb.get_step_mut("s1") { s.complete(10); }
    let tl = IncidentTimeline::new();
    let audit = IncidentAuditLog::new();
    let escalations: Vec<EscalationRecord> = vec![
        EscalationRecord::new("i1", 1, "alice", EscalationChannel::Pager, 1)
    ];
    let report = IncidentReport::generate(&i, Some(&rb), &tl, &audit, &escalations, 100);
    assert_eq!(report.runbook_steps_total, 2);
    assert_eq!(report.runbook_steps_done, 1);
    assert_eq!(report.escalation_count, 1);
    assert_eq!(report.runbook_progress(), 0.5);
}
