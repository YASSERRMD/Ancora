/// Categories of conduct violations in the extension ecosystem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ViolationCategory {
    Harassment,
    Spam,
    MaliciousCode,
    LicenseViolation,
    Other(String),
}

/// Status of a conduct report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReportStatus {
    Pending,
    UnderReview,
    Resolved,
    Dismissed,
}

/// A conduct report against a community member or extension.
#[derive(Debug, Clone)]
pub struct ConductReport {
    pub id: u64,
    pub reporter: String,
    pub accused: String,
    pub category: ViolationCategory,
    pub details: String,
    pub status: ReportStatus,
}

impl ConductReport {
    pub fn new(
        id: u64,
        reporter: impl Into<String>,
        accused: impl Into<String>,
        category: ViolationCategory,
        details: impl Into<String>,
    ) -> Self {
        ConductReport {
            id,
            reporter: reporter.into(),
            accused: accused.into(),
            category,
            details: details.into(),
            status: ReportStatus::Pending,
        }
    }

    /// Transition the report to UnderReview.
    pub fn begin_review(&mut self) -> Result<(), String> {
        if self.status == ReportStatus::Pending {
            self.status = ReportStatus::UnderReview;
            Ok(())
        } else {
            Err(format!("report is not pending: {:?}", self.status))
        }
    }

    /// Resolve or dismiss the report.
    pub fn close(&mut self, resolved: bool) -> Result<(), String> {
        if self.status == ReportStatus::UnderReview {
            self.status = if resolved {
                ReportStatus::Resolved
            } else {
                ReportStatus::Dismissed
            };
            Ok(())
        } else {
            Err(format!("report is not under review: {:?}", self.status))
        }
    }
}

/// Registry of conduct reports.
#[derive(Debug, Default)]
pub struct ConductRegistry {
    reports: Vec<ConductReport>,
}

impl ConductRegistry {
    pub fn new() -> Self {
        ConductRegistry {
            reports: Vec::new(),
        }
    }

    pub fn submit(&mut self, report: ConductReport) {
        self.reports.push(report);
    }

    pub fn pending_count(&self) -> usize {
        self.reports
            .iter()
            .filter(|r| r.status == ReportStatus::Pending)
            .count()
    }

    pub fn get_mut(&mut self, id: u64) -> Option<&mut ConductReport> {
        self.reports.iter_mut().find(|r| r.id == id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn report_lifecycle() {
        let mut report = ConductReport::new(
            1,
            "reporter-gh",
            "accused-gh",
            ViolationCategory::Spam,
            "Sent spam links in extension description",
        );
        report.begin_review().unwrap();
        report.close(true).unwrap();
        assert_eq!(report.status, ReportStatus::Resolved);
    }

    #[test]
    fn cannot_close_pending_report() {
        let mut report = ConductReport::new(2, "r", "a", ViolationCategory::Harassment, "Details");
        assert!(report.close(false).is_err());
    }
}
