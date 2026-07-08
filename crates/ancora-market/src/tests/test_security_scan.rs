use crate::security_scan::{Finding, ScanError, ScanReport, ScanStatus, Severity};

#[test]
fn clean_scan_is_attached_and_publishable() {
    let report = ScanReport::new(
        "ancora-scanner-v2",
        "2026-06-15T10:00:00Z",
        ScanStatus::Clean,
    );
    assert!(report.is_publishable().is_ok());
    assert_eq!(report.scanner, "ancora-scanner-v2");
}

#[test]
fn low_severity_finding_does_not_block() {
    let report = ScanReport::new(
        "ancora-scanner-v2",
        "2026-06-15T10:00:00Z",
        ScanStatus::Findings(vec![Finding {
            severity: Severity::Low,
            code: "INFO-001".to_string(),
            description: "Deprecated API usage".to_string(),
        }]),
    );
    assert!(report.is_publishable().is_ok());
}

#[test]
fn high_severity_finding_blocks() {
    let report = ScanReport::new(
        "ancora-scanner-v2",
        "2026-06-15T10:00:00Z",
        ScanStatus::Findings(vec![Finding {
            severity: Severity::High,
            code: "SEC-HIGH-001".to_string(),
            description: "SQL injection vulnerability".to_string(),
        }]),
    );
    assert!(matches!(
        report.is_publishable(),
        Err(ScanError::BlockingFindingsPresent(_))
    ));
}

#[test]
fn scan_error_blocks_publish() {
    let report = ScanReport::new(
        "ancora-scanner-v2",
        "2026-06-15T10:00:00Z",
        ScanStatus::Error("timeout after 30s".to_string()),
    );
    assert!(matches!(
        report.is_publishable(),
        Err(ScanError::ScanFailed(_))
    ));
}

#[test]
fn max_severity_reported_correctly() {
    let report = ScanReport::new(
        "ancora-scanner-v2",
        "2026-06-15T10:00:00Z",
        ScanStatus::Findings(vec![
            Finding {
                severity: Severity::Low,
                code: "L-001".to_string(),
                description: "Minor issue".to_string(),
            },
            Finding {
                severity: Severity::Medium,
                code: "M-001".to_string(),
                description: "Moderate issue".to_string(),
            },
        ]),
    );
    let max = report.max_severity().unwrap();
    assert_eq!(*max, Severity::Medium);
}
