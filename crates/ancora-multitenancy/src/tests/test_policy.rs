#[cfg(test)]
mod tests {
    use crate::{TenantConfig, TenantIsolation, TenantRegistry};

    fn registry_with_allowlist(providers: Vec<&str>) -> (TenantRegistry, crate::TenantId) {
        let mut reg = TenantRegistry::new();
        let cfg = TenantConfig {
            provider_allowlist: providers.into_iter().map(String::from).collect(),
            residency_region: Some("eu-west".to_string()),
            max_workers: 5,
        };
        let id = reg.create("acme", cfg);
        (reg, id)
    }

    #[test]
    fn allowed_provider_passes() {
        let (reg, id) = registry_with_allowlist(vec!["openai"]);
        let iso = TenantIsolation::new(&reg);
        assert!(iso.assert_provider_allowed(&id, "openai").is_ok());
    }

    #[test]
    fn disallowed_provider_blocked() {
        let (reg, id) = registry_with_allowlist(vec!["openai"]);
        let iso = TenantIsolation::new(&reg);
        assert!(iso.assert_provider_allowed(&id, "anthropic").is_err());
    }

    #[test]
    fn empty_allowlist_permits_all_providers() {
        let mut reg = TenantRegistry::new();
        let id = reg.create("acme", TenantConfig::default()); // default: empty allowlist
        let iso = TenantIsolation::new(&reg);
        assert!(iso.assert_provider_allowed(&id, "any-provider").is_ok());
    }

    #[test]
    fn residency_match_passes() {
        let (reg, id) = registry_with_allowlist(vec![]);
        let iso = TenantIsolation::new(&reg);
        assert!(iso.assert_residency(&id, "eu-west").is_ok());
    }

    #[test]
    fn residency_mismatch_blocked() {
        let (reg, id) = registry_with_allowlist(vec![]);
        let iso = TenantIsolation::new(&reg);
        assert!(iso.assert_residency(&id, "us-east").is_err());
    }

    #[test]
    fn no_residency_policy_allows_any_region() {
        let mut reg = TenantRegistry::new();
        let id = reg.create("acme", TenantConfig::default()); // no residency_region
        let iso = TenantIsolation::new(&reg);
        assert!(iso.assert_residency(&id, "us-east").is_ok());
    }
}
