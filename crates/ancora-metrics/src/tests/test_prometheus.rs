#[cfg(test)]
mod tests {
    use crate::{
        counter::RunCounters,
        gauge::{QueueDepthGauge, WorkerUtilizationGauge},
        histogram::Histogram,
        prometheus::{render_counters, render_histogram, render_queue_depth, render_worker_util},
    };

    #[test]
    fn counter_exposition_contains_metric_name() {
        let mut c = RunCounters::default();
        c.record_success("t1");
        let out = render_counters(&c, "t1");
        assert!(out.contains("ancora_run_success_total"));
        assert!(out.contains("tenant=\"t1\""));
        assert!(out.contains("} 1\n"));
    }

    #[test]
    fn histogram_exposition_contains_buckets() {
        let mut h = Histogram::new("ancora_run_latency", vec![10, 100]);
        h.observe(5);
        let out = render_histogram(&h);
        assert!(out.contains("_bucket{le=\"10\"} 1"));
        assert!(out.contains("_count 1"));
        assert!(out.contains("_sum "));
    }

    #[test]
    fn queue_depth_exposition_valid() {
        let mut g = QueueDepthGauge::default();
        g.set("t1", 7);
        let out = render_queue_depth(&g, "t1");
        assert!(out.contains("ancora_queue_depth{tenant=\"t1\"} 7"));
    }

    #[test]
    fn worker_util_exposition_valid() {
        let mut g = WorkerUtilizationGauge::default();
        g.set(2, 4);
        let out = render_worker_util(&g);
        assert!(out.contains("ancora_worker_utilization"));
        assert!(out.contains("0.5000"));
    }
}
