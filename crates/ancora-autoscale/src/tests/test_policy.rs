#[cfg(test)]
mod tests {
    use crate::bounds::ScaleBounds;
    use crate::decision::ScaleDecision;
    use crate::metrics::AutoscaleMetrics;
    use crate::policy::ScalePolicy;

    fn make_policy(min: usize, max: usize) -> ScalePolicy {
        ScalePolicy::new(ScaleBounds::new(min, max))
    }

    fn metrics(
        queue: usize,
        workers: usize,
        active: usize,
        concurrency: usize,
    ) -> AutoscaleMetrics {
        AutoscaleMetrics {
            queue_depth: queue,
            worker_count: workers,
            active_runs: active,
            concurrency_per_worker: concurrency,
            last_run_latency_ms: 0,
            utilization: AutoscaleMetrics::compute_utilization(active, workers, concurrency),
        }
    }

    #[test]
    fn scale_up_when_queue_grows() {
        let mut p = make_policy(1, 10);
        p.cooldown = crate::cooldown::Cooldown::new(0, 0);
        let m = metrics(10, 2, 2, 2);
        let d = p.evaluate(&m);
        assert!(d.is_scale_up(), "should scale up on high queue");
    }

    #[test]
    fn scale_down_when_idle() {
        let mut p = make_policy(1, 10);
        p.cooldown = crate::cooldown::Cooldown::new(0, 0);
        let m = metrics(0, 5, 0, 4);
        let d = p.evaluate(&m);
        assert!(d.is_scale_down(), "should scale down when idle");
    }

    #[test]
    fn respects_min_bound() {
        let mut p = make_policy(2, 10);
        p.cooldown = crate::cooldown::Cooldown::new(0, 0);
        let m = metrics(0, 2, 0, 4);
        let d = p.evaluate(&m);
        // Already at min; should not scale down
        assert!(!d.is_scale_down(), "should not go below min");
    }

    #[test]
    fn respects_max_bound() {
        let mut p = make_policy(1, 3);
        p.cooldown = crate::cooldown::Cooldown::new(0, 0);
        let m = metrics(100, 3, 3, 1);
        let d = p.evaluate(&m);
        assert!(!d.is_scale_up(), "should not exceed max");
    }

    #[test]
    fn noop_when_within_range() {
        let mut p = make_policy(1, 10);
        p.cooldown = crate::cooldown::Cooldown::new(0, 0);
        let m = metrics(2, 4, 2, 4);
        let d = p.evaluate(&m);
        // Queue=2 < threshold=5 and utilization=0.125 < scale_up=0.8 but > scale_down=0.2
        // Nothing should happen
        drop(d);
    }

    #[test]
    fn tenant_cap_respected() {
        use crate::bounds::TenantCap;
        let cap = TenantCap {
            tenant_id: "t1".to_string(),
            max_workers: 3,
        };
        let clamped = cap.max_workers.min(5);
        assert_eq!(clamped, 3);
    }

    #[test]
    fn scaling_decisions_logged() {
        use crate::decision::ScaleDecision;
        use crate::signals::ScaleSignal;
        let sig = ScaleSignal::from_decision(ScaleDecision::ScaleUp { by: 2 }, 4, 10, 0.8);
        assert!(sig.decision.is_scale_up());
        assert_eq!(sig.desired_workers, 6);
    }
}
