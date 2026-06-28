use crate::incident::{Incident, Severity};
use crate::timeline::IncidentTimeline;
use crate::audit::IncidentAuditLog;
use crate::report::IncidentReport;

#[test]
fn report_progress_zero_steps() {
    let i = Incident::new("i1", "t1", "T", Severity::Low, 0);
    let tl = IncidentTimeline::new();
    let audit = IncidentAuditLog::new();
    let r = IncidentReport::generate(&i, None, &tl, &audit, &[], 100);
    assert_eq!(r.runbook_progress(), 0.0);
}

#[test]
fn report_duration() {
    let mut i = Incident::new("i1", "t1", "T", Severity::High, 50);
    i.resolve(150);
    let tl = IncidentTimeline::new();
    let audit = IncidentAuditLog::new();
    let r = IncidentReport::generate(&i, None, &tl, &audit, &[], 200);
    assert_eq!(r.duration_ticks, 100);
}
