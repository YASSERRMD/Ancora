use ancora_airgap::audit::{AirGapAction, AirGapAuditEntry, AirGapAuditLog};
use ancora_airgap::boundary::{AirGapBoundary, AirGapZone, ZoneClassification};
use ancora_airgap::media::MediaType;
use ancora_airgap::policy::PolicyVerdict;
use ancora_airgap::presets::{data_import_procedure, restricted_zone, strict_airgap_policy};
use ancora_airgap::report::AirGapReport;
use ancora_airgap::store::TransferStore;
use ancora_airgap::transfer::{TransferDirection, TransferRequest};

fn main() {
    let policy = strict_airgap_policy("tenant-1");

    let usb_req = TransferRequest::new(
        "tx-001",
        "tenant-1",
        "alice",
        MediaType::UsbDrive,
        TransferDirection::Inbound,
        "firmware update",
        1,
    )
    .with_checksum("sha256:deadbeef");

    let bluetooth_req = TransferRequest::new(
        "tx-002",
        "tenant-1",
        "bob",
        MediaType::Bluetooth,
        TransferDirection::Inbound,
        "file sync",
        2,
    );

    let usb_verdict = policy.evaluate(&usb_req);
    let bt_verdict = policy.evaluate(&bluetooth_req);
    println!("USB transfer verdict: {:?}", usb_verdict);
    println!("Bluetooth verdict: {:?}", bt_verdict);

    assert!(matches!(usb_verdict, PolicyVerdict::RequireApproval));
    assert!(matches!(bt_verdict, PolicyVerdict::Deny(_)));

    let mut boundary = AirGapBoundary::new();
    boundary.add_zone(restricted_zone("tenant-1"));
    boundary.add_zone(AirGapZone::new(
        "pub-zone",
        "Public Zone",
        ZoneClassification::Public,
        "tenant-1",
    ));
    println!("Total zones: {}", boundary.count());
    println!("Restricted zones: {}", boundary.restricted_zones().len());

    let mut store = TransferStore::new();
    store.insert(usb_req);

    let mut audit = AirGapAuditLog::new();
    audit.record(AirGapAuditEntry::new(
        1,
        "tenant-1",
        AirGapAction::TransferRequested,
        "alice",
        "usb import",
    ));
    audit.record(AirGapAuditEntry::new(
        2,
        "tenant-1",
        AirGapAction::MediaBlocked,
        "sys",
        "bluetooth blocked",
    ));

    let report = AirGapReport::generate(&boundary, &store, &audit, 100);
    println!(
        "Report: zones={} restricted={} transfers={} pending={} audit={}",
        report.total_zones,
        report.restricted_zones,
        report.total_transfers,
        report.pending_transfers,
        report.total_audit_entries
    );

    let mut proc = data_import_procedure("tenant-1");
    proc.get_step_mut("s1").unwrap().complete(10);
    proc.get_step_mut("s2").unwrap().complete(20);
    println!("Import procedure progress: {:.0}%", proc.progress() * 100.0);
}
