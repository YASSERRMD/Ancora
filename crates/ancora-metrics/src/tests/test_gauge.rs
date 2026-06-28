#[cfg(test)]
mod tests {
    use crate::gauge::{QueueDepthGauge, WorkerUtilizationGauge};

    #[test]
    fn queue_depth_set_and_get() {
        let mut g = QueueDepthGauge::default();
        g.set("t1", 5);
        assert_eq!(g.get("t1"), 5);
    }

    #[test]
    fn queue_depth_missing_tenant_is_zero() {
        let g = QueueDepthGauge::default();
        assert_eq!(g.get("nobody"), 0);
    }

    #[test]
    fn worker_utilization_computed() {
        let mut g = WorkerUtilizationGauge::default();
        g.set(3, 4);
        assert!((g.utilization() - 0.75).abs() < 1e-9);
    }

    #[test]
    fn worker_utilization_zero_total() {
        let g = WorkerUtilizationGauge::default();
        assert_eq!(g.utilization(), 0.0);
    }
}
