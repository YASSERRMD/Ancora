/// Badge issuance for compliant extensions.

use crate::report::{KitReport, KitStatus};

/// Tier of compliance badge.
#[derive(Debug, Clone, PartialEq)]
pub enum BadgeTier {
    /// All checks passed.
    Compliant,
    /// More than half passed.
    PartiallyCompliant,
    /// Half or fewer passed.
    NonCompliant,
}

impl std::fmt::Display for BadgeTier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BadgeTier::Compliant => write!(f, "Ancora ITK: Compliant"),
            BadgeTier::PartiallyCompliant => write!(f, "Ancora ITK: Partially Compliant"),
            BadgeTier::NonCompliant => write!(f, "Ancora ITK: Non-Compliant"),
        }
    }
}

/// An issued badge for an extension.
#[derive(Debug, Clone)]
pub struct Badge {
    pub extension_name: String,
    pub tier: BadgeTier,
    pub passed: usize,
    pub total: usize,
}

impl Badge {
    /// Build a badge from a [`KitReport`].
    pub fn from_report(extension_name: impl Into<String>, report: &KitReport) -> Self {
        let tier = match &report.status {
            KitStatus::AllPassed => BadgeTier::Compliant,
            KitStatus::SomeFailed { failed, total } => {
                if *failed < total / 2 {
                    BadgeTier::PartiallyCompliant
                } else {
                    BadgeTier::NonCompliant
                }
            }
        };
        Badge {
            extension_name: extension_name.into(),
            tier,
            passed: report.passed,
            total: report.total,
        }
    }

    /// Return true only for fully compliant extensions.
    pub fn is_compliant(&self) -> bool {
        self.tier == BadgeTier::Compliant
    }

    /// Render a one-line badge string.
    pub fn render(&self) -> String {
        format!(
            "[{}] {} ({}/{})",
            self.tier, self.extension_name, self.passed, self.total
        )
    }
}

/// Issue a badge for the given report; returns None if the report is empty.
pub fn issue_badge(extension_name: impl Into<String>, report: &KitReport) -> Option<Badge> {
    if report.total == 0 {
        return None;
    }
    Some(Badge::from_report(extension_name, report))
}
