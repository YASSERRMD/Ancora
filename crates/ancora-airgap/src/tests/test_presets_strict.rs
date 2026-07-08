use crate::media::MediaType;
use crate::policy::PolicyVerdict;
use crate::presets::strict_airgap_policy;
use crate::transfer::{TransferDirection, TransferRequest};

#[test]
fn strict_blocks_outbound() {
    let policy = strict_airgap_policy("t1");
    let req = TransferRequest::new(
        "r1",
        "t1",
        "alice",
        MediaType::PrintedDocument,
        TransferDirection::Outbound,
        "",
        1,
    )
    .with_checksum("sha256:abc");
    assert!(matches!(policy.evaluate(&req), PolicyVerdict::Deny(_)));
}

#[test]
fn strict_blocks_network_share() {
    let policy = strict_airgap_policy("t1");
    let req = TransferRequest::new(
        "r1",
        "t1",
        "alice",
        MediaType::NetworkShare,
        TransferDirection::Inbound,
        "",
        1,
    )
    .with_checksum("sha256:abc");
    assert!(matches!(policy.evaluate(&req), PolicyVerdict::Deny(_)));
}

#[test]
fn strict_requires_checksum_on_usb() {
    let policy = strict_airgap_policy("t1");
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
