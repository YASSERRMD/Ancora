#[cfg(test)]
mod tests {
    use crate::{CanaryController, Version, VersionedWorker};

    fn ctrl_with_pct(pct: f64) -> CanaryController {
        let stable = vec![VersionedWorker::new("s1", Version::new(1, 0, 0))];
        let canary = vec![VersionedWorker::new("c1", Version::new(2, 0, 0))];
        CanaryController::new(stable, canary, pct, 10.0)
    }

    #[test]
    fn canary_rollback_then_new_canary_is_clean() {
        let mut c = ctrl_with_pct(0.2);
        for _ in 0..5 { c.record_canary_result(true); } // trigger failure
        c.rollback(); // clear canary state
        // Start fresh canary
        c.canary = vec![VersionedWorker::new("c2", Version::new(2, 1, 0))];
        c.canary_pct = 0.2;
        // New canary has clean counters
        assert!(c.check_health_gate().is_ok(), "fresh canary should pass health gate");
    }

    #[test]
    fn zero_pct_canary_routes_no_traffic() {
        let c = ctrl_with_pct(0.0);
        let canary_hits: usize = (0u64..1000).filter(|i| c.route_to_canary(*i)).count();
        assert_eq!(canary_hits, 0);
    }
}
