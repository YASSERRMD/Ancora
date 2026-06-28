use crate::media::MediaType;
use crate::policy::{AirGapPolicy, PolicyVerdict};
use crate::transfer::{TransferDirection, TransferRequest};

fn make_req(media: MediaType, direction: TransferDirection) -> TransferRequest {
    TransferRequest::new("r1", "t1", "alice", media, direction, "test", 1)
}

#[test]
fn policy_allow_by_default() {
    let policy = AirGapPolicy::new("t1");
    let req = make_req(MediaType::PrintedDocument, TransferDirection::Inbound);
    assert_eq!(policy.evaluate(&req), PolicyVerdict::Allow);
}

#[test]
fn policy_block_media() {
    let policy = AirGapPolicy::new("t1").block_media(MediaType::Bluetooth);
    let req = make_req(MediaType::Bluetooth, TransferDirection::Inbound);
    assert!(matches!(policy.evaluate(&req), PolicyVerdict::Deny(_)));
}

#[test]
fn policy_require_approval() {
    let policy = AirGapPolicy::new("t1").require_approval_for(MediaType::UsbDrive);
    let req = make_req(MediaType::UsbDrive, TransferDirection::Inbound);
    assert_eq!(policy.evaluate(&req), PolicyVerdict::RequireApproval);
}

#[test]
fn policy_block_all_outbound() {
    let policy = AirGapPolicy::new("t1").block_all_outbound();
    let req = make_req(MediaType::PrintedDocument, TransferDirection::Outbound);
    assert!(matches!(policy.evaluate(&req), PolicyVerdict::Deny(_)));
}

#[test]
fn policy_require_checksum() {
    let policy = AirGapPolicy::new("t1").require_checksum();
    let req = make_req(MediaType::UsbDrive, TransferDirection::Inbound);
    assert!(matches!(policy.evaluate(&req), PolicyVerdict::Deny(_)));
}
