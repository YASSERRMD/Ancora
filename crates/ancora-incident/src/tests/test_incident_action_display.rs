use crate::audit::IncidentAction;

#[test]
fn action_display() {
    assert_eq!(format!("{}", IncidentAction::Created), "CREATED");
    assert_eq!(
        format!("{}", IncidentAction::StatusUpdated),
        "STATUS_UPDATED"
    );
    assert_eq!(format!("{}", IncidentAction::Assigned), "ASSIGNED");
    assert_eq!(format!("{}", IncidentAction::Escalated), "ESCALATED");
    assert_eq!(
        format!("{}", IncidentAction::RunbookStarted),
        "RUNBOOK_STARTED"
    );
    assert_eq!(
        format!("{}", IncidentAction::RunbookStepDone),
        "RUNBOOK_STEP_DONE"
    );
    assert_eq!(format!("{}", IncidentAction::Resolved), "RESOLVED");
    assert_eq!(
        format!("{}", IncidentAction::PostmortemCreated),
        "POSTMORTEM_CREATED"
    );
}
