use crate::boundary::ZoneClassification;
use crate::builder::{TransferBuilder, ZoneBuilder};
use crate::media::MediaType;
use crate::transfer::{TransferDirection, TransferStatus};

#[test]
fn transfer_builder_defaults() {
    let req = TransferBuilder::new(
        "r1",
        "t1",
        "alice",
        MediaType::UsbDrive,
        TransferDirection::Inbound,
    )
    .build();
    assert_eq!(req.status, TransferStatus::Pending);
    assert!(req.checksum.is_none());
}

#[test]
fn transfer_builder_with_checksum() {
    let req = TransferBuilder::new(
        "r1",
        "t1",
        "alice",
        MediaType::CdRom,
        TransferDirection::Outbound,
    )
    .tick(100)
    .checksum("sha256:deadbeef")
    .build();
    assert_eq!(req.checksum.as_deref(), Some("sha256:deadbeef"));
    assert_eq!(req.created_tick, 100);
}

#[test]
fn zone_builder() {
    let zone = ZoneBuilder::new("z1", "Primary Zone", ZoneClassification::Restricted, "t1").build();
    assert!(zone.is_restricted());
    assert_eq!(zone.id, "z1");
}
