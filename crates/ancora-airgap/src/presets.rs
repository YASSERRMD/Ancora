use crate::boundary::{AirGapZone, ZoneClassification};
use crate::media::MediaType;
use crate::policy::AirGapPolicy;
use crate::procedure::{OfflineProcedure, ProcedureStep};

pub fn strict_airgap_policy(tenant_id: impl Into<String> + Clone) -> AirGapPolicy {
    AirGapPolicy::new(tenant_id)
        .block_media(MediaType::Bluetooth)
        .block_media(MediaType::NetworkShare)
        .require_approval_for(MediaType::UsbDrive)
        .require_checksum()
        .block_all_outbound()
}

pub fn standard_airgap_policy(tenant_id: impl Into<String>) -> AirGapPolicy {
    AirGapPolicy::new(tenant_id)
        .block_media(MediaType::Bluetooth)
        .block_media(MediaType::NetworkShare)
        .require_approval_for(MediaType::UsbDrive)
}

pub fn restricted_zone(tenant_id: impl Into<String>) -> AirGapZone {
    AirGapZone::new(
        "restricted-zone-1",
        "Restricted Operations Zone",
        ZoneClassification::Restricted,
        tenant_id,
    )
}

pub fn top_secret_zone(tenant_id: impl Into<String>) -> AirGapZone {
    AirGapZone::new(
        "ts-zone-1",
        "Top Secret Zone",
        ZoneClassification::TopSecret,
        tenant_id,
    )
}

pub fn data_import_procedure(tenant_id: impl Into<String>) -> OfflineProcedure {
    let mut proc = OfflineProcedure::new("data-import-1", "Offline Data Import", tenant_id);
    proc.add_step(ProcedureStep::new(
        "s1",
        "Verify media integrity",
        "Check checksum of source media",
    ));
    proc.add_step(ProcedureStep::new(
        "s2",
        "Scan for malware",
        "Run offline antivirus scan on media",
    ));
    proc.add_step(ProcedureStep::new(
        "s3",
        "Obtain approval signature",
        "Get written approval from security officer",
    ));
    proc.add_step(ProcedureStep::new(
        "s4",
        "Transfer data",
        "Copy data from media to isolated system",
    ));
    proc.add_step(ProcedureStep::new(
        "s5",
        "Log transfer",
        "Record transfer in offline audit log",
    ));
    proc
}
