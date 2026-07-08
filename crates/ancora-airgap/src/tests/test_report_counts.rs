use crate::audit::{AirGapAction, AirGapAuditEntry, AirGapAuditLog};
use crate::boundary::{AirGapBoundary, AirGapZone, ZoneClassification};
use crate::media::MediaType;
use crate::report::AirGapReport;
use crate::store::TransferStore;
use crate::transfer::{TransferDirection, TransferRequest};

#[test]
fn report_audit_entries() {
    let b = AirGapBoundary::new();
    let s = TransferStore::new();
    let mut a = AirGapAuditLog::new();
    a.record(AirGapAuditEntry::new(
        1,
        "t1",
        AirGapAction::TransferRequested,
        "alice",
        "",
    ));
    a.record(AirGapAuditEntry::new(
        2,
        "t1",
        AirGapAction::TransferApproved,
        "bob",
        "",
    ));
    let r = AirGapReport::generate(&b, &s, &a, 50);
    assert_eq!(r.total_audit_entries, 2);
}

#[test]
fn report_restricted_zone_count() {
    let mut b = AirGapBoundary::new();
    b.add_zone(AirGapZone::new(
        "z1",
        "Z1",
        ZoneClassification::TopSecret,
        "t1",
    ));
    b.add_zone(AirGapZone::new(
        "z2",
        "Z2",
        ZoneClassification::Restricted,
        "t1",
    ));
    b.add_zone(AirGapZone::new(
        "z3",
        "Z3",
        ZoneClassification::Public,
        "t1",
    ));
    let s = TransferStore::new();
    let a = AirGapAuditLog::new();
    let r = AirGapReport::generate(&b, &s, &a, 1);
    assert_eq!(r.total_zones, 3);
    assert_eq!(r.restricted_zones, 2);
}

#[test]
fn report_pending_transfers() {
    let b = AirGapBoundary::new();
    let mut s = TransferStore::new();
    let req1 = TransferRequest::new(
        "r1",
        "t1",
        "alice",
        MediaType::UsbDrive,
        TransferDirection::Inbound,
        "",
        1,
    );
    let mut req2 = TransferRequest::new(
        "r2",
        "t1",
        "alice",
        MediaType::CdRom,
        TransferDirection::Inbound,
        "",
        1,
    );
    req2.approve(5);
    s.insert(req1);
    s.insert(req2);
    let a = AirGapAuditLog::new();
    let r = AirGapReport::generate(&b, &s, &a, 10);
    assert_eq!(r.total_transfers, 2);
    assert_eq!(r.pending_transfers, 1);
}
