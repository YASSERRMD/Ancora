use crate::incident::{Incident, Severity};
use crate::runbook::{Runbook, RunbookStep};
use crate::timeline::{IncidentTimeline, TimelineEvent, TimelineEventKind};
use crate::postmortem::Postmortem;

#[test]
fn postmortem_no_runbook() {
    let mut i = Incident::new("i1", "t1", "Test", Severity::High, 100);
    i.resolve(200);
    let tl = IncidentTimeline::new();
    let pm = Postmortem::generate(&i, None, &tl, 200, "Unknown", "Patched");
    assert_eq!(pm.incident_id, "i1");
    assert_eq!(pm.steps_total, 0);
    assert_eq!(pm.runbook_completion_rate(), 0.0);
    assert_eq!(pm.duration_ticks, 100);
}

#[test]
fn postmortem_with_runbook() {
    let i = Incident::new("i1", "t1", "Test", Severity::Critical, 0);
    let mut rb = Runbook::new("rb1", "Test", "i1");
    rb.add_step(RunbookStep::new("s1", "A", "Desc"));
    rb.add_step(RunbookStep::new("s2", "B", "Desc"));
    if let Some(s) = rb.get_step_mut("s1") { s.complete(5); }
    let tl = IncidentTimeline::new();
    let pm = Postmortem::generate(&i, Some(&rb), &tl, 100, "Bug", "Fix");
    assert_eq!(pm.steps_total, 2);
    assert_eq!(pm.steps_completed, 1);
    assert_eq!(pm.runbook_completion_rate(), 0.5);
}

#[test]
fn postmortem_timeline_events() {
    let i = Incident::new("i1", "t1", "Test", Severity::Low, 0);
    let mut tl = IncidentTimeline::new();
    tl.add(TimelineEvent::new("i1", TimelineEventKind::Note, "u", "Note", 1));
    tl.add(TimelineEvent::new("i1", TimelineEventKind::Note, "u", "Note2", 2));
    tl.add(TimelineEvent::new("i2", TimelineEventKind::Note, "u", "Other", 3));
    let pm = Postmortem::generate(&i, None, &tl, 100, "R", "R");
    assert_eq!(pm.timeline_event_count, 2);
}
