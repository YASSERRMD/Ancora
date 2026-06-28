#[cfg(test)]
mod tests {
    use crate::{TenantConfig, TenantIsolation, TenantRegistry, TenantState};

    #[test]
    fn suspended_tenant_runs_rejected() {
        let mut reg = TenantRegistry::new();
        let id = reg.create("acme", TenantConfig::default());
        reg.suspend(&id).unwrap();
        let iso = TenantIsolation::new(&reg);
        let err = iso.assert_active(&id).unwrap_err();
        assert!(err.to_string().contains("suspended"));
    }

    #[test]
    fn deleted_tenant_runs_rejected() {
        let mut reg = TenantRegistry::new();
        let id = reg.create("acme", TenantConfig::default());
        reg.delete(&id).unwrap();
        let iso = TenantIsolation::new(&reg);
        assert!(iso.assert_active(&id).is_err());
    }

    #[test]
    fn tenant_state_transitions_correctly() {
        let mut reg = TenantRegistry::new();
        let id = reg.create("acme", TenantConfig::default());
        assert_eq!(reg.get(&id).unwrap().state, TenantState::Active);
        reg.suspend(&id).unwrap();
        assert_eq!(reg.get(&id).unwrap().state, TenantState::Suspended);
        reg.delete(&id).unwrap();
        assert_eq!(reg.get(&id).unwrap().state, TenantState::Deleted);
    }

    #[test]
    fn tenant_list_includes_all_states() {
        let mut reg = TenantRegistry::new();
        reg.create("a", TenantConfig::default());
        reg.create("b", TenantConfig::default());
        assert_eq!(reg.list().len(), 2);
    }
}
