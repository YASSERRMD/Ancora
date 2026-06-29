use crate::boot::BootStatus;
use crate::report::{AttestationReport, ReportStatus};

#[test]
fn test_attestation_report_generated() {
    let report = AttestationReport::generate(
        "device-007",
        100,
        BootStatus::Verified,
        true,
        true,
        vec![],
        42,
    );
    assert_eq!(report.status, ReportStatus::Clean);
    assert!(report.is_clean());
    assert_eq!(report.tamper_count(), 0);
}

#[test]
fn test_attestation_report_compromised() {
    use crate::tamper::{TamperEvent, TamperEventKind};
    let events = vec![TamperEvent::new("device-007", TamperEventKind::HashMismatch, "hash", 1)];
    let report = AttestationReport::generate(
        "device-007",
        101,
        BootStatus::Verified,
        true,
        true,
        events,
        43,
    );
    assert_eq!(report.status, ReportStatus::Compromised);
    assert!(!report.is_clean());
}

#[test]
fn test_attestation_report_warning_model_invalid() {
    let report = AttestationReport::generate(
        "device-008",
        102,
        BootStatus::Verified,
        false,
        true,
        vec![],
        44,
    );
    assert_eq!(report.status, ReportStatus::Warning);
}

#[test]
fn test_attestation_report_text_contains_device_id() {
    let report = AttestationReport::generate(
        "edge-99",
        200,
        BootStatus::Verified,
        true,
        true,
        vec![],
        99,
    );
    let text = report.to_text();
    assert!(text.contains("edge-99"));
    assert!(text.contains("VERIFIED"));
    assert!(text.contains("CLEAN"));
}
