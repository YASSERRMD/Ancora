//! Governance and versioning policy for the Ancora ecosystem.
//!
//! Documents the stability guarantees, deprecation timeline,
//! and breaking-change policy that Ancora maintainers follow.

/// Stability tier for a public API surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Stability {
    /// Experimental: may change in any release without notice.
    Experimental = 0,
    /// Unstable: may change between minor versions; documented in release notes.
    Unstable = 1,
    /// Stable: breaking changes only in major version bumps.
    Stable = 2,
    /// Deprecated: will be removed in the next major version.
    Deprecated = 3,
}

impl std::fmt::Display for Stability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            Self::Experimental => "experimental",
            Self::Unstable => "unstable",
            Self::Stable => "stable",
            Self::Deprecated => "deprecated",
        };
        write!(f, "{label}")
    }
}

/// Semantic version components.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SemVer {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl SemVer {
    pub const fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Returns `true` if bumping from `self` to `next` is a breaking change.
    pub fn is_breaking_from(&self, next: &SemVer) -> bool {
        next.major > self.major
    }
}

impl std::fmt::Display for SemVer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// Minimum deprecation notice period in minor releases.
pub const DEPRECATION_NOTICE_RELEASES: u32 = 2;

/// Returns the minimum release in which a symbol may be removed
/// given the release it was first deprecated.
pub fn earliest_removal(deprecated_in: SemVer) -> SemVer {
    SemVer::new(deprecated_in.major + 1, 0, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn major_bump_is_breaking() {
        let old = SemVer::new(1, 4, 0);
        let next = SemVer::new(2, 0, 0);
        assert!(old.is_breaking_from(&next));
    }

    #[test]
    fn minor_bump_is_not_breaking() {
        let old = SemVer::new(1, 0, 0);
        let next = SemVer::new(1, 1, 0);
        assert!(!old.is_breaking_from(&next));
    }

    #[test]
    fn earliest_removal_is_next_major() {
        let deprecated = SemVer::new(1, 3, 0);
        let removal = earliest_removal(deprecated);
        assert_eq!(removal.major, 2);
        assert_eq!(removal.minor, 0);
    }

    #[test]
    fn stability_ordering() {
        assert!(Stability::Stable > Stability::Unstable);
        assert!(Stability::Unstable > Stability::Experimental);
    }
}
