use crate::transfer::TransferStatus;

#[test]
fn display_pending() {
    assert_eq!(format!("{}", TransferStatus::Pending), "PENDING");
}

#[test]
fn display_approved() {
    assert_eq!(format!("{}", TransferStatus::Approved), "APPROVED");
}

#[test]
fn display_rejected() {
    assert_eq!(format!("{}", TransferStatus::Rejected), "REJECTED");
}

#[test]
fn display_completed() {
    assert_eq!(format!("{}", TransferStatus::Completed), "COMPLETED");
}

#[test]
fn display_cancelled() {
    assert_eq!(format!("{}", TransferStatus::Cancelled), "CANCELLED");
}
