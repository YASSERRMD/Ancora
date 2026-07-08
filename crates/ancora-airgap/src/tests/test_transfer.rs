use crate::media::MediaType;
use crate::transfer::{TransferDirection, TransferRequest, TransferStatus};

#[test]
fn transfer_initial_pending() {
    let r = TransferRequest::new(
        "r1",
        "t1",
        "alice",
        MediaType::UsbDrive,
        TransferDirection::Inbound,
        "import",
        1,
    );
    assert!(r.is_pending());
    assert_eq!(r.status, TransferStatus::Pending);
}

#[test]
fn transfer_approve() {
    let mut r = TransferRequest::new(
        "r1",
        "t1",
        "alice",
        MediaType::UsbDrive,
        TransferDirection::Inbound,
        "import",
        1,
    );
    r.approve(10);
    assert!(r.is_approved());
    assert_eq!(r.resolved_tick, Some(10));
}

#[test]
fn transfer_reject() {
    let mut r = TransferRequest::new(
        "r1",
        "t1",
        "alice",
        MediaType::CdRom,
        TransferDirection::Outbound,
        "export",
        1,
    );
    r.reject(5);
    assert_eq!(r.status, TransferStatus::Rejected);
}

#[test]
fn transfer_with_checksum() {
    let r = TransferRequest::new(
        "r1",
        "t1",
        "alice",
        MediaType::UsbDrive,
        TransferDirection::Inbound,
        "",
        1,
    )
    .with_checksum("sha256:abc123");
    assert_eq!(r.checksum.as_deref(), Some("sha256:abc123"));
}
