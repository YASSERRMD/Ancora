use crate::media::MediaType;
use crate::policy::{AirGapPolicy, PolicyVerdict};
use crate::transfer::{TransferDirection, TransferRequest};

#[test]
fn block_takes_priority_over_approval() {
    let policy = AirGapPolicy::new("t1")
        .block_media(MediaType::UsbDrive)
        .require_approval_for(MediaType::UsbDrive);
    let req = TransferRequest::new(
        "r1",
        "t1",
        "alice",
        MediaType::UsbDrive,
        TransferDirection::Inbound,
        "",
        1,
    );
    assert!(matches!(policy.evaluate(&req), PolicyVerdict::Deny(_)));
}

#[test]
fn checksum_required_but_provided() {
    let policy = AirGapPolicy::new("t1").require_checksum();
    let req = TransferRequest::new(
        "r1",
        "t1",
        "alice",
        MediaType::PrintedDocument,
        TransferDirection::Inbound,
        "",
        1,
    )
    .with_checksum("sha256:abc");
    assert_eq!(policy.evaluate(&req), PolicyVerdict::Allow);
}

#[test]
fn media_blocked_check() {
    let policy = AirGapPolicy::new("t1").block_media(MediaType::Bluetooth);
    assert!(policy.media_blocked(&MediaType::Bluetooth));
    assert!(!policy.media_blocked(&MediaType::UsbDrive));
}
