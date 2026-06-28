#[cfg(test)]
mod tests {
    use crate::bounds::TenantCap;
    use crate::decision::ScaleDecision;
    use crate::tenant_policy::TenantPolicyEngine;

    #[test]
    fn tenant_cap_enforced_on_scale_up() {
        let mut e = TenantPolicyEngine::new();
        e.set_cap(TenantCap { tenant_id: "t1".to_string(), max_workers: 3 });
        e.set_workers("t1", 3);
        let result = e.apply("t1", ScaleDecision::ScaleUp { by: 2 });
        assert!(result.is_noop(), "should be noop when already at cap");
    }

    #[test]
    fn tenant_cap_allows_partial_scale_up() {
        let mut e = TenantPolicyEngine::new();
        e.set_cap(TenantCap { tenant_id: "t1".to_string(), max_workers: 5 });
        e.set_workers("t1", 4);
        let result = e.apply("t1", ScaleDecision::ScaleUp { by: 3 });
        match result {
            ScaleDecision::ScaleUp { by } => assert_eq!(by, 1),
            _ => panic!("expected ScaleUp with by=1"),
        }
    }

    #[test]
    fn no_cap_passes_through_decision() {
        let e = TenantPolicyEngine::new();
        let result = e.apply("uncapped", ScaleDecision::ScaleUp { by: 5 });
        assert!(result.is_scale_up());
    }

    #[test]
    fn scale_down_not_affected_by_cap() {
        let mut e = TenantPolicyEngine::new();
        e.set_cap(TenantCap { tenant_id: "t1".to_string(), max_workers: 3 });
        let result = e.apply("t1", ScaleDecision::ScaleDown { by: 1 });
        assert!(result.is_scale_down());
    }
}
