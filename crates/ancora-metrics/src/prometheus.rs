use crate::{
    counter::RunCounters,
    gauge::{QueueDepthGauge, WorkerUtilizationGauge},
    histogram::Histogram,
};

/// Render metrics in Prometheus text exposition format.
pub fn render_counters(counters: &RunCounters, tenant: &str) -> String {
    format!(
        "# HELP ancora_run_success_total Total successful runs\n\
         # TYPE ancora_run_success_total counter\n\
         ancora_run_success_total{{tenant=\"{tenant}\"}} {}\n\
         # HELP ancora_run_failure_total Total failed runs\n\
         # TYPE ancora_run_failure_total counter\n\
         ancora_run_failure_total{{tenant=\"{tenant}\"}} {}\n",
        counters.success_total(tenant),
        counters.failure_total(tenant),
    )
}

pub fn render_histogram(h: &Histogram) -> String {
    let mut out = format!(
        "# HELP {} Latency histogram\n\
         # TYPE {} histogram\n",
        h.label, h.label
    );
    for &bound in &h.buckets {
        out.push_str(&format!(
            "{}_bucket{{le=\"{bound}\"}} {}\n",
            h.label,
            h.bucket_count(bound)
        ));
    }
    out.push_str(&format!("{}_sum {}\n", h.label, h.sum_ms()));
    out.push_str(&format!("{}_count {}\n", h.label, h.count()));
    out
}

pub fn render_queue_depth(gauge: &QueueDepthGauge, tenant: &str) -> String {
    format!(
        "# HELP ancora_queue_depth Current queued run count\n\
         # TYPE ancora_queue_depth gauge\n\
         ancora_queue_depth{{tenant=\"{tenant}\"}} {}\n",
        gauge.get(tenant)
    )
}

pub fn render_worker_util(gauge: &WorkerUtilizationGauge) -> String {
    format!(
        "# HELP ancora_worker_utilization Fraction of workers busy\n\
         # TYPE ancora_worker_utilization gauge\n\
         ancora_worker_utilization {:.4}\n",
        gauge.utilization()
    )
}
