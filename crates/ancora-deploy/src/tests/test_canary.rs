#[cfg(test)]
mod tests {
    use crate::{CanaryController, Version, VersionedWorker};

    fn ctrl() -> CanaryController {
        let stable = vec![VersionedWorker::new("s1", Version::new(1, 0, 0))];
        let canary = vec![VersionedWorker::new("c1", Version::new(2, 0, 0))];
        CanaryController::new(stable, canary, 0.1, 5.0)
    }

    #[test]
    fn canary_subset_receives_partial_traffic() {
        let c = ctrl();
        // With pct=0.1, period=10; every 10th request goes to canary
        assert!(c.route_to_canary(0));
        assert!(!c.route_to_canary(1));
        assert!(!c.route_to_canary(5));
        assert!(c.route_to_canary(10));
    }

    #[test]
    fn canary_failure_triggers_rollback() {
        let mut c = ctrl();
        for _ in 0..10 { c.record_canary_result(true); } // 100% error rate
        let err = c.check_health_gate().unwrap_err();
        assert!(matches!(err, crate::DeployError::CanaryHealthGateFailed { .. }));
        c.rollback();
        assert!(c.canary.is_empty());
    }

    #[test]
    fn healthy_canary_passes_gate() {
        let mut c = ctrl();
        for _ in 0..100 { c.record_canary_result(false); }
        c.record_canary_result(true); // 1% errors - under threshold of 5%
        assert!(c.check_health_gate().is_ok());
    }

    #[test]
    fn canary_promotion_makes_canary_stable() {
        let mut c = ctrl();
        c.promote();
        assert_eq!(c.stable.first().unwrap().version.major, 2);
        assert!(c.canary.is_empty());
        assert_eq!(c.canary_pct, 0.0);
    }

    #[test]
    fn mixed_version_workers_interoperate_via_routing() {
        let c = ctrl();
        // Stable (v1) and canary (v2) both present; routing is deterministic
        let canary_count = (0u64..100).filter(|i| c.route_to_canary(*i)).count();
        assert!(canary_count > 0 && canary_count < 100);
    }
}
