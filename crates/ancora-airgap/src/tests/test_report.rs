use crate::audit::AirGapAuditLog;
use crate::boundary::{AirGapBoundary, AirGapZone, ZoneClassification};
use crate::media::MediaType;
use crate::report::AirGapReport;
use crate::store::TransferStore;
use crate::transfer::{TransferDirection, TransferRequest};

#[test]
fn report_empty() {
    let b = AirGapBoundary::new();
    let s = TransferStore::new();
    let a = AirGapAuditLog::new();
    let r = AirGapReport::generate(&b, &s, &a, 1);
    assert_eq!(r.total_zones, 0);
    assert_eq!(r.total_transfers, 0);
}

#[test]
fn report_with_data() {
    let mut b = AirGapBoundary::new();
    b.add_zone(AirGapZone::new("z1", "Z1", ZoneClassification::Restricted, "t1"));
    b.add_zone(AirGapZone::new("z2", "Z2", ZoneClassification::Public, "t1"));
    let mut s = TransferStore::new();
    s.insert(TransferRequest::new("r1", "t1", "alice", MediaType::UsbDrive, TransferDirection::Inbound, "", 1));
    let a = AirGapAuditLog::new();
    let r = AirGapReport::generate(&b, &s, &a, 10);
    assert_eq!(r.total_zones, 2);
    assert_eq!(r.restricted_zones, 1);
    assert_eq!(r.total_transfers, 1);
    assert_eq!(r.pending_transfers, 1);
    assert_eq!(r.tick, 10);
}
