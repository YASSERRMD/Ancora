//! Ecosystem readiness checklist for Ancora.
//!
//! Evaluates whether a plugin or the broader ecosystem is ready
//! for a particular milestone (e.g., v1.0 stability, marketplace listing).

/// Readiness milestone.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Milestone {
    AlphaRelease,
    BetaRelease,
    StableRelease,
    MarketplaceListing,
}

impl std::fmt::Display for Milestone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            Self::AlphaRelease => "alpha-release",
            Self::BetaRelease => "beta-release",
            Self::StableRelease => "stable-release",
            Self::MarketplaceListing => "marketplace-listing",
        };
        write!(f, "{label}")
    }
}

/// A readiness criterion.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Criterion {
    pub id: &'static str,
    pub description: &'static str,
    pub required_for: Vec<Milestone>,
}

/// The full readiness checklist.
pub fn readiness_criteria() -> Vec<Criterion> {
    vec![
        Criterion {
            id: "ready-001",
            description: "Crate compiles on stable Rust without warnings",
            required_for: vec![
                Milestone::AlphaRelease,
                Milestone::BetaRelease,
                Milestone::StableRelease,
                Milestone::MarketplaceListing,
            ],
        },
        Criterion {
            id: "ready-002",
            description: "All public items are documented with `///` comments",
            required_for: vec![
                Milestone::BetaRelease,
                Milestone::StableRelease,
                Milestone::MarketplaceListing,
            ],
        },
        Criterion {
            id: "ready-003",
            description: "Test coverage covers happy path and at least two error cases",
            required_for: vec![
                Milestone::AlphaRelease,
                Milestone::BetaRelease,
                Milestone::StableRelease,
                Milestone::MarketplaceListing,
            ],
        },
        Criterion {
            id: "ready-004",
            description: "Catalog entry passes validation",
            required_for: vec![Milestone::MarketplaceListing],
        },
        Criterion {
            id: "ready-005",
            description: "No unsafe code, or unsafe blocks are documented and reviewed",
            required_for: vec![Milestone::StableRelease, Milestone::MarketplaceListing],
        },
        Criterion {
            id: "ready-006",
            description: "CHANGELOG.md contains an entry for the current version",
            required_for: vec![
                Milestone::BetaRelease,
                Milestone::StableRelease,
                Milestone::MarketplaceListing,
            ],
        },
    ]
}

/// Returns the criteria required for a given milestone.
pub fn criteria_for(milestone: &Milestone) -> Vec<Criterion> {
    readiness_criteria()
        .into_iter()
        .filter(|c| c.required_for.contains(milestone))
        .collect()
}

/// Checks whether the provided set of satisfied criterion IDs meets the milestone.
pub fn is_ready(milestone: &Milestone, satisfied: &[&str]) -> Result<(), Vec<&'static str>> {
    let missing: Vec<&'static str> = criteria_for(milestone)
        .iter()
        .filter(|c| !satisfied.contains(&c.id))
        .map(|c| c.id)
        .collect();
    if missing.is_empty() {
        Ok(())
    } else {
        Err(missing)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn criteria_are_non_empty() {
        assert!(!readiness_criteria().is_empty());
    }

    #[test]
    fn marketplace_requires_catalog_entry() {
        let criteria = criteria_for(&Milestone::MarketplaceListing);
        assert!(criteria.iter().any(|c| c.id == "ready-004"));
    }

    #[test]
    fn alpha_does_not_require_docs() {
        let criteria = criteria_for(&Milestone::AlphaRelease);
        assert!(!criteria.iter().any(|c| c.id == "ready-002"));
    }

    #[test]
    fn is_ready_passes_when_all_satisfied() {
        let milestone = Milestone::AlphaRelease;
        let ids: Vec<&str> = criteria_for(&milestone).iter().map(|c| c.id).collect();
        assert!(is_ready(&milestone, &ids).is_ok());
    }

    #[test]
    fn is_ready_fails_when_some_missing() {
        assert!(is_ready(&Milestone::AlphaRelease, &[]).is_err());
    }
}
