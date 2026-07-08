use crate::media::MediaType;
use crate::store::TransferStore;
use crate::transfer::{TransferDirection, TransferRequest, TransferStatus};

#[test]
fn pending_decreases_after_approval() {
    let mut s = TransferStore::new();
    s.insert(TransferRequest::new(
        "r1",
        "t1",
        "alice",
        MediaType::UsbDrive,
        TransferDirection::Inbound,
        "",
        1,
    ));
    s.insert(TransferRequest::new(
        "r2",
        "t1",
        "alice",
        MediaType::CdRom,
        TransferDirection::Inbound,
        "",
        1,
    ));
    s.get_mut("r1").unwrap().approve(5);
    assert_eq!(s.pending().len(), 1);
    assert_eq!(s.by_status(&TransferStatus::Approved).len(), 1);
}

#[test]
fn remove_transfer() {
    let mut s = TransferStore::new();
    s.insert(TransferRequest::new(
        "r1",
        "t1",
        "alice",
        MediaType::UsbDrive,
        TransferDirection::Inbound,
        "",
        1,
    ));
    assert!(s.remove("r1").is_some());
    assert_eq!(s.count(), 0);
}
