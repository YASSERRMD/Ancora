use crate::media::MediaType;
use crate::stats::AirGapStats;
use crate::transfer::{TransferDirection, TransferRequest};

#[test]
fn rejection_rate_zero() {
    let r1 = TransferRequest::new("r1", "t1", "alice", MediaType::UsbDrive, TransferDirection::Inbound, "", 1);
    let v: Vec<&TransferRequest> = vec![&r1];
    let s = AirGapStats::for_tenant(&v, "t1");
    assert_eq!(s.rejection_rate(), 0.0);
}

#[test]
fn rejection_rate_half() {
    let r1 = TransferRequest::new("r1", "t1", "alice", MediaType::UsbDrive, TransferDirection::Inbound, "", 1);
    let mut r2 = TransferRequest::new("r2", "t1", "alice", MediaType::UsbDrive, TransferDirection::Inbound, "", 1);
    r2.reject(5);
    let v: Vec<&TransferRequest> = vec![&r1, &r2];
    let s = AirGapStats::for_tenant(&v, "t1");
    assert_eq!(s.rejection_rate(), 0.5);
}

#[test]
fn stats_only_counts_matching_tenant() {
    let r1 = TransferRequest::new("r1", "t1", "alice", MediaType::UsbDrive, TransferDirection::Inbound, "", 1);
    let r2 = TransferRequest::new("r2", "t2", "bob", MediaType::UsbDrive, TransferDirection::Inbound, "", 1);
    let v: Vec<&TransferRequest> = vec![&r1, &r2];
    let s = AirGapStats::for_tenant(&v, "t1");
    assert_eq!(s.total_transfers, 1);
}
