/// Stability levels for extension API points.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum StabilityLevel {
    /// Not stable; may change or be removed without notice.
    Unstable,
    /// Stable for use but not covered by long-term guarantees.
    Experimental,
    /// Stable; breaking changes require a deprecation period.
    Stable,
    /// Will not change; frozen in the specification.
    Frozen,
}

/// Policy governing when a stability level permits a breaking change.
#[derive(Debug, Clone)]
pub struct StabilityPolicy {
    pub min_deprecation_cycles: u32,
    pub requires_rfc: bool,
}

impl StabilityPolicy {
    /// Return the policy for a given stability level.
    pub fn for_level(level: &StabilityLevel) -> Self {
        match level {
            StabilityLevel::Unstable => StabilityPolicy {
                min_deprecation_cycles: 0,
                requires_rfc: false,
            },
            StabilityLevel::Experimental => StabilityPolicy {
                min_deprecation_cycles: 1,
                requires_rfc: false,
            },
            StabilityLevel::Stable => StabilityPolicy {
                min_deprecation_cycles: 2,
                requires_rfc: true,
            },
            StabilityLevel::Frozen => StabilityPolicy {
                min_deprecation_cycles: u32::MAX,
                requires_rfc: true,
            },
        }
    }

    /// Returns true if a breaking change is allowed given current deprecation cycle count.
    pub fn allows_breaking_change(&self, elapsed_cycles: u32) -> bool {
        elapsed_cycles >= self.min_deprecation_cycles
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unstable_allows_immediate_break() {
        let policy = StabilityPolicy::for_level(&StabilityLevel::Unstable);
        assert!(policy.allows_breaking_change(0));
    }

    #[test]
    fn stable_requires_two_cycles() {
        let policy = StabilityPolicy::for_level(&StabilityLevel::Stable);
        assert!(!policy.allows_breaking_change(1));
        assert!(policy.allows_breaking_change(2));
    }
}
