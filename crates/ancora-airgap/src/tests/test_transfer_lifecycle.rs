use crate::media::MediaType;
use crate::transfer::{TransferDirection, TransferRequest, TransferStatus};

#[test]
fn complete_after_approve() {
    let mut r = TransferRequest::new("r1", "t1", "alice", MediaType::UsbDrive, TransferDirection::Inbound, "", 1);
    r.approve(5);
    r.complete(10);
    assert_eq!(r.status, TransferStatus::Completed);
    assert_eq!(r.resolved_tick, Some(10));
}

#[test]
fn cancel() {
    let mut r = TransferRequest::new("r1", "t1", "alice", MediaType::UsbDrive, TransferDirection::Inbound, "", 1);
    r.cancel();
    assert_eq!(r.status, TransferStatus::Cancelled);
}

#[test]
fn not_pending_after_approve() {
    let mut r = TransferRequest::new("r1", "t1", "alice", MediaType::CdRom, TransferDirection::Outbound, "", 1);
    r.approve(2);
    assert!(!r.is_pending());
    assert!(r.is_approved());
}
