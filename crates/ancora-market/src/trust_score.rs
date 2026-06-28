/// Trust score computation for marketplace extensions.
///
/// A trust score from 0-100 is computed from a set of signals. Higher scores
/// indicate more trustworthy extensions. The score is used by the install
/// policy to gate or warn on installs.

use crate::badge::BadgeSet;
use crate::identity::AuthorIdentity;
use crate::license::LicenseDeclaration;
use crate::residency::ResidencyDeclaration;
use crate::security_scan::{ScanReport, ScanStatus, Severity};

/// Input signals used to compute the trust score.
pub struct TrustSignals<'a> {
    pub author: &'a AuthorIdentity,
    pub scan_report: Option<&'a ScanReport>,
    pub license: Option<&'a LicenseDeclaration>,
    pub residency: Option<&'a ResidencyDeclaration>,
    pub badges: &'a BadgeSet,
    /// Number of distinct versions published (history depth).
    pub version_count: u32,
}

/// Weighted trust score result.
#[derive(Debug, Clone, PartialEq)]
pub struct TrustScore {
    /// Final score in range [0, 100].
    pub score: u32,
    /// Breakdown by category.
    pub breakdown: TrustBreakdown,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TrustBreakdown {
    /// Points from identity verification (max 20).
    pub identity: u32,
    /// Points from security scan (max 30).
    pub security: u32,
    /// Points from license declaration (max 15).
    pub license: u32,
    /// Points from residency declaration (max 15).
    pub residency: u32,
    /// Points from badges (max 10).
    pub badges: u32,
    /// Points from version history (max 10).
    pub history: u32,
}

impl TrustScore {
    pub fn is_high_trust(&self) -> bool {
        self.score >= 80
    }

    pub fn is_acceptable(&self) -> bool {
        self.score >= 50
    }
}

/// Compute the trust score for an extension from its signals.
pub fn compute_trust_score(signals: &TrustSignals<'_>) -> TrustScore {
    // Identity: 20 points - 20 for verified, 5 for unverified
    let identity = if signals.author.verified { 20 } else { 5 };

    // Security scan: 30 points
    let security = match &signals.scan_report {
        None => 0,
        Some(r) => match &r.status {
            ScanStatus::Clean => 30,
            ScanStatus::Error(_) => 0,
            ScanStatus::Findings(findings) => {
                let has_critical = findings.iter().any(|f| f.severity == Severity::Critical);
                let has_high = findings.iter().any(|f| f.severity == Severity::High);
                if has_critical { 0 } else if has_high { 5 } else { 20 }
            }
        },
    };

    // License: 15 points - 15 for open-source, 5 for declared-but-proprietary
    let license = match &signals.license {
        None => 0,
        Some(l) => {
            if l.is_open_source { 15 } else { 5 }
        }
    };

    // Residency: 15 points - 15 if complete, 5 if declared but incomplete
    let residency = match &signals.residency {
        None => 0,
        Some(r) => {
            if r.is_complete() { 15 } else { 5 }
        }
    };

    // Badges: 2 points each, max 10
    let badge_pts = (signals.badges.count() as u32) * 2;
    let badges = badge_pts.min(10);

    // History: 2 points per version, max 10
    let history = (signals.version_count * 2).min(10);

    let total = identity + security + license + residency + badges + history;
    let score = total.min(100);

    TrustScore {
        score,
        breakdown: TrustBreakdown {
            identity,
            security,
            license,
            residency,
            badges,
            history,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::identity::AuthorIdentity;
    use crate::security_scan::{ScanReport, ScanStatus};
    use crate::license::LicenseDeclaration;
    use crate::residency::{Region, ResidencyDeclaration};

    #[test]
    fn perfect_score() {
        let mut author = AuthorIdentity::new("alice", "Alice", "PUBKEY").unwrap();
        author.mark_verified();
        let scan = ScanReport::new("scanner", "2026-01-01", ScanStatus::Clean);
        let license = LicenseDeclaration::new("Apache-2.0", true).unwrap();
        let residency = ResidencyDeclaration::new(vec![Region::EEA], vec![Region::EEA]);
        let mut badges = BadgeSet::new();
        use crate::badge::{Badge, BadgeKind};
        badges.award(Badge {
            kind: BadgeKind::SecurityVerified,
            awarded_on: "2026-01-01".to_string(),
            issuer: "ancora-registry".to_string(),
        });
        let signals = TrustSignals {
            author: &author,
            scan_report: Some(&scan),
            license: Some(&license),
            residency: Some(&residency),
            badges: &badges,
            version_count: 5,
        };
        let score = compute_trust_score(&signals);
        assert!(score.score >= 80);
        assert!(score.is_high_trust());
    }

    #[test]
    fn unverified_no_scan_low_score() {
        let author = AuthorIdentity::new("anon", "Anon", "PUBKEY").unwrap();
        let badges = BadgeSet::new();
        let signals = TrustSignals {
            author: &author,
            scan_report: None,
            license: None,
            residency: None,
            badges: &badges,
            version_count: 0,
        };
        let score = compute_trust_score(&signals);
        assert!(!score.is_high_trust());
        assert!(!score.is_acceptable());
    }
}
