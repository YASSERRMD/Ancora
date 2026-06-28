//! Marketplace trust signals for the Ancora plugin ecosystem.
//!
//! Aggregates signals such as download counts, review scores,
//! and security audit status to compute a trust score.

/// Security audit status for a plugin release.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditStatus {
    NotAudited,
    Pending,
    Passed,
    Failed,
}

/// Aggregated trust signals for a single plugin version.
#[derive(Debug, Clone)]
pub struct TrustSignals {
    pub download_count: u64,
    pub review_score: f32,
    pub review_count: u32,
    pub audit_status: AuditStatus,
    pub maintained: bool,
}

impl TrustSignals {
    /// Compute a normalized trust score in `[0.0, 1.0]`.
    pub fn score(&self) -> f32 {
        let mut score = 0.0_f32;

        // Download popularity (max contribution: 0.3)
        let popularity = (self.download_count as f32 / 10_000.0).min(1.0) * 0.3;
        score += popularity;

        // Review score, weighted by count (max contribution: 0.3)
        if self.review_count > 0 {
            let review_weight = (self.review_count as f32 / 50.0).min(1.0);
            let normalized = (self.review_score / 5.0).clamp(0.0, 1.0);
            score += normalized * review_weight * 0.3;
        }

        // Audit status (max contribution: 0.3)
        score += match self.audit_status {
            AuditStatus::Passed => 0.3,
            AuditStatus::Pending => 0.1,
            AuditStatus::NotAudited => 0.0,
            AuditStatus::Failed => -0.2,
        };

        // Active maintenance bonus (max: 0.1)
        if self.maintained {
            score += 0.1;
        }

        score.clamp(0.0, 1.0)
    }

    /// Returns `true` if this plugin should be shown in the featured section.
    pub fn is_featured(&self) -> bool {
        self.score() >= 0.7 && self.audit_status == AuditStatus::Passed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn high_trust() -> TrustSignals {
        TrustSignals {
            download_count: 50_000,
            review_score: 4.8,
            review_count: 120,
            audit_status: AuditStatus::Passed,
            maintained: true,
        }
    }

    #[test]
    fn high_trust_score_is_featured() {
        let s = high_trust();
        assert!(s.score() >= 0.7);
        assert!(s.is_featured());
    }

    #[test]
    fn failed_audit_lowers_score() {
        let mut s = high_trust();
        s.audit_status = AuditStatus::Failed;
        assert!(!s.is_featured());
    }

    #[test]
    fn score_is_bounded() {
        let s = high_trust();
        let score = s.score();
        assert!((0.0..=1.0).contains(&score));
    }

    #[test]
    fn zero_downloads_contributes_zero_popularity() {
        let s = TrustSignals {
            download_count: 0,
            review_score: 0.0,
            review_count: 0,
            audit_status: AuditStatus::NotAudited,
            maintained: false,
        };
        assert_eq!(s.score(), 0.0);
    }
}
