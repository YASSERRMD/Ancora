use crate::media::MediaType;
use crate::stats::AirGapStats;
use crate::transfer::{TransferDirection, TransferRequest};

fn make(id: &str) -> TransferRequest {
    TransferRequest::new(
        id,
        "t1",
        "alice",
        MediaType::UsbDrive,
        TransferDirection::Inbound,
        "",
        1,
    )
}

#[test]
fn stats_empty() {
    let s = AirGapStats::for_tenant(&[], "t1");
    assert_eq!(s.total_transfers, 0);
    assert_eq!(s.rejection_rate(), 0.0);
}

#[test]
fn stats_counts() {
    let r1 = make("r1");
    let mut r2 = make("r2");
    r2.reject(5);
    let v: Vec<&TransferRequest> = vec![&r1, &r2];
    let s = AirGapStats::for_tenant(&v, "t1");
    assert_eq!(s.total_transfers, 2);
    assert_eq!(s.pending, 1);
    assert_eq!(s.rejected, 1);
}

#[test]
fn stats_by_media() {
    let r1 = make("r1");
    let v: Vec<&TransferRequest> = vec![&r1];
    let s = AirGapStats::for_tenant(&v, "t1");
    assert_eq!(s.by_media.get("USB_DRIVE").copied(), Some(1));
}
