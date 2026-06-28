use crate::timeline::TimelineEventKind;

#[test]
fn event_kind_display() {
    assert_eq!(format!("{}", TimelineEventKind::Detected), "DETECTED");
    assert_eq!(format!("{}", TimelineEventKind::Assigned), "ASSIGNED");
    assert_eq!(format!("{}", TimelineEventKind::StatusChanged), "STATUS_CHANGED");
    assert_eq!(format!("{}", TimelineEventKind::RunbookStepCompleted), "RUNBOOK_STEP_COMPLETED");
    assert_eq!(format!("{}", TimelineEventKind::EscalationTriggered), "ESCALATION_TRIGGERED");
    assert_eq!(format!("{}", TimelineEventKind::Note), "NOTE");
    assert_eq!(format!("{}", TimelineEventKind::Resolved), "RESOLVED");
}
