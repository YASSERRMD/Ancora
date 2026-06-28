/// Data-residency enforcement for plugins.
///
/// Plugins may only transfer data to geographic regions that are explicitly
/// permitted by the residency policy. This supports compliance with GDPR,
/// CCPA, and other data-sovereignty regulations.

/// An identifier for a geographic region (e.g., "eu-west", "us-east").
pub type RegionCode = String;

/// Describes which regions are permitted for data transfer.
#[derive(Debug, Clone)]
pub struct ResidencyPolicy {
    /// The set of permitted region codes.  `None` means all regions are
    /// permitted (global policy).  An empty `Some` means no region is
    /// permitted.
    allowed_regions: Option<Vec<RegionCode>>,
}

/// An error produced when a data transfer violates the residency policy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResidencyViolation {
    /// The target region is not in the allowed set.
    RegionNotPermitted { region: RegionCode },
}

impl std::fmt::Display for ResidencyViolation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RegionNotPermitted { region } =>
                write!(f, "data transfer to region '{}' is not permitted by residency policy", region),
        }
    }
}

impl ResidencyPolicy {
    /// Allow data transfers only to the specified regions.
    pub fn allow_only(regions: Vec<RegionCode>) -> Self {
        Self { allowed_regions: Some(regions) }
    }

    /// Allow data transfers to any region (no residency restriction).
    pub fn global() -> Self {
        Self { allowed_regions: None }
    }

    /// Deny all data transfers regardless of region.
    pub fn deny_all() -> Self {
        Self { allowed_regions: Some(vec![]) }
    }

    /// Check whether a data transfer to `region` is permitted.
    pub fn permits_transfer(&self, region: &str) -> Result<(), ResidencyViolation> {
        match &self.allowed_regions {
            // Global policy: all regions permitted.
            None => Ok(()),
            // Restricted policy: check allow-list.
            Some(allowed) => {
                if allowed.iter().any(|r| r == region) {
                    Ok(())
                } else {
                    Err(ResidencyViolation::RegionNotPermitted { region: region.to_string() })
                }
            }
        }
    }

    /// Returns the list of explicitly allowed regions, or `None` for global.
    pub fn allowed_regions(&self) -> Option<&[RegionCode]> {
        self.allowed_regions.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn global_policy_allows_any_region() {
        let policy = ResidencyPolicy::global();
        assert!(policy.permits_transfer("eu-west").is_ok());
        assert!(policy.permits_transfer("us-east").is_ok());
    }

    #[test]
    fn deny_all_rejects_every_region() {
        let policy = ResidencyPolicy::deny_all();
        assert!(policy.permits_transfer("eu-west").is_err());
    }

    #[test]
    fn allow_only_permits_listed_regions() {
        let policy = ResidencyPolicy::allow_only(vec!["eu-west".into(), "us-east".into()]);
        assert!(policy.permits_transfer("eu-west").is_ok());
        assert!(policy.permits_transfer("ap-southeast").is_err());
    }
}
