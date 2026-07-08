use crate::audit::AirGapAction;

#[test]
fn display_transfer_requested() {
    assert_eq!(
        format!("{}", AirGapAction::TransferRequested),
        "TRANSFER_REQUESTED"
    );
}

#[test]
fn display_transfer_approved() {
    assert_eq!(
        format!("{}", AirGapAction::TransferApproved),
        "TRANSFER_APPROVED"
    );
}

#[test]
fn display_transfer_rejected() {
    assert_eq!(
        format!("{}", AirGapAction::TransferRejected),
        "TRANSFER_REJECTED"
    );
}

#[test]
fn display_transfer_completed() {
    assert_eq!(
        format!("{}", AirGapAction::TransferCompleted),
        "TRANSFER_COMPLETED"
    );
}

#[test]
fn display_media_blocked() {
    assert_eq!(format!("{}", AirGapAction::MediaBlocked), "MEDIA_BLOCKED");
}

#[test]
fn display_procedure_completed() {
    assert_eq!(
        format!("{}", AirGapAction::ProcedureCompleted),
        "PROCEDURE_COMPLETED"
    );
}
