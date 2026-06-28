/// Security scan results attached to a marketplace extension.
///
/// Before an extension can be published, it must pass an automated security
/// scan. The scan result is attached to the extension manifest and is
/// re-verified on install.

#[derive(Debug, Clone, PartialEq)]
pub enum ScanStatus {
    /// All checks passed with no findings.
    Clean,
    /// Scan identified issues at the given severity.
    Findings(Vec<Finding>),
    /// Scan could not be completed.
    Error(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::Low => write!(f, "low"),
            Severity::Medium => write!(f, "medium"),
            Severity::High => write!(f, "high"),
            Severity::Critical => write!(f, "critical"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Finding {
    pub severity: Severity,
    pub code: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct ScanReport {
    /// Scanner tool name and version.
    pub scanner: String,
    /// ISO-8601 timestamp of the scan.
    pub scanned_at: String,
    pub status: ScanStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ScanError {
    MissingScanReport,
    ScanFailed(String),
    BlockingFindingsPresent(Vec<String>),
}

impl std::fmt::Display for ScanError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScanError::MissingScanReport => write!(f, "no security scan report attached"),
            ScanError::ScanFailed(msg) => write!(f, "scan failed: {}", msg),
            ScanError::BlockingFindingsPresent(codes) => {
                write!(f, "blocking findings: {}", codes.join(", "))
            }
        }
    }
}

impl ScanReport {
    pub fn new(scanner: impl Into<String>, scanned_at: impl Into<String>, status: ScanStatus) -> Self {
        ScanReport {
            scanner: scanner.into(),
            scanned_at: scanned_at.into(),
            status,
        }
    }

    /// Return whether the scan result is acceptable for publishing.
    /// Critical and High severity findings block publishing.
    pub fn is_publishable(&self) -> Result<(), ScanError> {
        match &self.status {
            ScanStatus::Clean => Ok(()),
            ScanStatus::Error(msg) => Err(ScanError::ScanFailed(msg.clone())),
            ScanStatus::Findings(findings) => {
                let blocking: Vec<String> = findings
                    .iter()
                    .filter(|f| {
                        matches!(f.severity, Severity::High | Severity::Critical)
                    })
                    .map(|f| f.code.clone())
                    .collect();
                if blocking.is_empty() {
                    Ok(())
                } else {
                    Err(ScanError::BlockingFindingsPresent(blocking))
                }
            }
        }
    }

    /// Maximum severity present in findings.
    pub fn max_severity(&self) -> Option<&Severity> {
        match &self.status {
            ScanStatus::Findings(findings) => findings.iter().map(|f| &f.severity).max_by_key(|s| {
                match s {
                    Severity::Low => 0,
                    Severity::Medium => 1,
                    Severity::High => 2,
                    Severity::Critical => 3,
                }
            }),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clean_scan_is_publishable() {
        let r = ScanReport::new("scanner-v1", "2026-01-01T00:00:00Z", ScanStatus::Clean);
        assert!(r.is_publishable().is_ok());
    }

    #[test]
    fn critical_finding_blocks_publish() {
        let r = ScanReport::new(
            "scanner-v1",
            "2026-01-01T00:00:00Z",
            ScanStatus::Findings(vec![Finding {
                severity: Severity::Critical,
                code: "CVE-2026-9999".to_string(),
                description: "Remote code execution".to_string(),
            }]),
        );
        assert!(matches!(
            r.is_publishable(),
            Err(ScanError::BlockingFindingsPresent(_))
        ));
    }
}
