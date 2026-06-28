use crate::media::MediaType;
use crate::store::TransferStore;
use crate::transfer::{TransferDirection, TransferRequest, TransferStatus};

fn make(id: &str, tenant_id: &str) -> TransferRequest {
    TransferRequest::new(id, tenant_id, "alice", MediaType::UsbDrive, TransferDirection::Inbound, "", 1)
}

#[test]
fn store_insert_and_get() {
    let mut s = TransferStore::new();
    s.insert(make("r1", "t1"));
    assert!(s.get("r1").is_some());
    assert_eq!(s.count(), 1);
}

#[test]
fn store_for_tenant() {
    let mut s = TransferStore::new();
    s.insert(make("r1", "t1"));
    s.insert(make("r2", "t2"));
    assert_eq!(s.for_tenant("t1").len(), 1);
}

#[test]
fn store_pending() {
    let mut s = TransferStore::new();
    s.insert(make("r1", "t1"));
    let mut r2 = make("r2", "t1");
    r2.approve(5);
    s.insert(r2);
    assert_eq!(s.pending().len(), 1);
}

#[test]
fn store_by_status() {
    let mut s = TransferStore::new();
    s.insert(make("r1", "t1"));
    assert_eq!(s.by_status(&TransferStatus::Pending).len(), 1);
}
