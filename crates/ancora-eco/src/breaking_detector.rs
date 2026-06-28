use crate::semver::SemVer;
use crate::stability_policy::{StabilityLevel, StabilityPolicy};

/// Represents an API signature snapshot used for diffing.
#[derive(Debug, Clone)]
pub struct ApiSnapshot {
    pub version: SemVer,
    pub endpoints: Vec<String>,
}

impl ApiSnapshot {
    pub fn new(version: SemVer, endpoints: Vec<impl Into<String>>) -> Self {
        ApiSnapshot {
            version,
            endpoints: endpoints.into_iter().map(|e| e.into()).collect(),
        }
    }
}

/// A detected breaking change between two snapshots.
#[derive(Debug, Clone)]
pub struct BreakingChange {
    /// The endpoint or symbol that was removed or incompatibly changed.
    pub item: String,
    pub description: String,
}

/// Compare two snapshots and return all detected breaking changes.
/// A breaking change is any endpoint present in `old` but absent in `new`.
pub fn detect_breaking_changes(old: &ApiSnapshot, new: &ApiSnapshot) -> Vec<BreakingChange> {
    let mut changes = Vec::new();
    for endpoint in &old.endpoints {
        if !new.endpoints.contains(endpoint) {
            changes.push(BreakingChange {
                item: endpoint.clone(),
                description: format!("`{}` was removed", endpoint),
            });
        }
    }
    changes
}

/// CI check: returns Err if breaking changes are detected and the stability
/// policy does not permit them.
pub fn ci_check_stability(
    old: &ApiSnapshot,
    new: &ApiSnapshot,
    level: &StabilityLevel,
    elapsed_deprecation_cycles: u32,
) -> Result<(), Vec<BreakingChange>> {
    let changes = detect_breaking_changes(old, new);
    if changes.is_empty() {
        return Ok(());
    }
    let policy = StabilityPolicy::for_level(level);
    if policy.allows_breaking_change(elapsed_deprecation_cycles) {
        Ok(())
    } else {
        Err(changes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn removed_endpoint_detected() {
        let old = ApiSnapshot::new(SemVer::new(1, 0, 0), vec!["foo", "bar"]);
        let new = ApiSnapshot::new(SemVer::new(1, 1, 0), vec!["foo"]);
        let changes = detect_breaking_changes(&old, &new);
        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0].item, "bar");
    }

    #[test]
    fn no_changes_when_endpoints_identical() {
        let old = ApiSnapshot::new(SemVer::new(1, 0, 0), vec!["foo"]);
        let new = ApiSnapshot::new(SemVer::new(1, 1, 0), vec!["foo"]);
        assert!(detect_breaking_changes(&old, &new).is_empty());
    }

    #[test]
    fn ci_fails_on_stable_without_cycles() {
        let old = ApiSnapshot::new(SemVer::new(1, 0, 0), vec!["foo", "bar"]);
        let new = ApiSnapshot::new(SemVer::new(2, 0, 0), vec!["foo"]);
        let result = ci_check_stability(&old, &new, &StabilityLevel::Stable, 0);
        assert!(result.is_err());
    }
}
