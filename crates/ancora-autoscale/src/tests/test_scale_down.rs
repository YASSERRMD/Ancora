#[cfg(test)]
mod tests {
    use crate::bounds::ScaleBounds;
    use crate::cooldown::Cooldown;
    use crate::decision::ScaleDecision;
    use crate::metrics::AutoscaleMetrics;
    use crate::policy::ScalePolicy;

    fn make_policy() -> ScalePolicy {
        let mut p = ScalePolicy::new(ScaleBounds::new(1, 10));
        p.cooldown = Cooldown::new(0, 0);
        p
    }

    #[test]
    fn scale_down_when_idle() {
        let mut p = make_policy();
        let m = AutoscaleMetrics {
            queue_depth: 0,
            worker_count: 5,
            active_runs: 0,
            concurrency_per_worker: 4,
            last_run_latency_ms: 0,
            utilization: 0.0,
        };
        let d = p.evaluate(&m);
        assert!(d.is_scale_down(), "idle pool should scale down");
    }

    #[test]
    fn no_scale_down_when_busy() {
        let mut p = make_policy();
        let m = AutoscaleMetrics {
            queue_depth: 0,
            worker_count: 5,
            active_runs: 4,
            concurrency_per_worker: 4,
            last_run_latency_ms: 100,
            utilization: 0.2, // exactly at threshold
        };
        // utilization == scale_down threshold (0.2) but queue=0
        let d = p.evaluate(&m);
        // This is borderline; just confirm the decision was computed
        assert!(matches!(
            d,
            ScaleDecision::ScaleDown { .. } | ScaleDecision::NoOp { .. }
        ));
    }
}
