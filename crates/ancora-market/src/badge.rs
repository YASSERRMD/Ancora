/// Conformance badge integration for marketplace extensions.
///
/// Badges signal that an extension has passed a specific quality gate.
/// They are awarded by the registry and embedded in extension metadata.

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BadgeKind {
    /// Extension has passed the security scan.
    SecurityVerified,
    /// Extension has a declared and recognized open-source license.
    OpenSourceLicense,
    /// Extension has declared data-residency information.
    ResidencyDeclared,
    /// Extension has a trust score >= 80.
    HighTrust,
    /// Extension has been reviewed by the marketplace team.
    OfficiallyReviewed,
}

impl std::fmt::Display for BadgeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BadgeKind::SecurityVerified => write!(f, "security-verified"),
            BadgeKind::OpenSourceLicense => write!(f, "open-source-license"),
            BadgeKind::ResidencyDeclared => write!(f, "residency-declared"),
            BadgeKind::HighTrust => write!(f, "high-trust"),
            BadgeKind::OfficiallyReviewed => write!(f, "officially-reviewed"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Badge {
    pub kind: BadgeKind,
    /// ISO-8601 date when the badge was awarded, e.g. "2026-01-15".
    pub awarded_on: String,
    /// Issuing authority (usually "ancora-registry").
    pub issuer: String,
}

/// Collection of badges attached to an extension.
#[derive(Debug, Clone, Default)]
pub struct BadgeSet {
    badges: Vec<Badge>,
}

impl BadgeSet {
    pub fn new() -> Self {
        BadgeSet { badges: Vec::new() }
    }

    /// Add a badge; duplicates (same kind) are silently ignored.
    pub fn award(&mut self, badge: Badge) {
        if !self.has(&badge.kind) {
            self.badges.push(badge);
        }
    }

    /// Check whether a badge of a given kind is present.
    pub fn has(&self, kind: &BadgeKind) -> bool {
        self.badges.iter().any(|b| &b.kind == kind)
    }

    /// Return all awarded badges.
    pub fn all(&self) -> &[Badge] {
        &self.badges
    }

    /// Number of distinct badges awarded.
    pub fn count(&self) -> usize {
        self.badges.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn award_and_check() {
        let mut set = BadgeSet::new();
        assert!(!set.has(&BadgeKind::SecurityVerified));
        set.award(Badge {
            kind: BadgeKind::SecurityVerified,
            awarded_on: "2026-01-01".to_string(),
            issuer: "ancora-registry".to_string(),
        });
        assert!(set.has(&BadgeKind::SecurityVerified));
    }

    #[test]
    fn no_duplicates() {
        let mut set = BadgeSet::new();
        let badge = Badge {
            kind: BadgeKind::HighTrust,
            awarded_on: "2026-01-01".to_string(),
            issuer: "ancora-registry".to_string(),
        };
        set.award(badge.clone());
        set.award(badge);
        assert_eq!(set.count(), 1);
    }
}
