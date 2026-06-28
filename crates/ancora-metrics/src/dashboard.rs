use serde_json::{json, Value};

/// Generate a minimal Grafana dashboard JSON for Ancora SLOs.
pub fn grafana_dashboard() -> Value {
    json!({
        "title": "Ancora SLO Dashboard",
        "uid": "ancora-slo-v1",
        "schemaVersion": 36,
        "panels": [
            {
                "type": "timeseries",
                "title": "Run Success Rate",
                "targets": [{
                    "expr": "rate(ancora_run_success_total[5m]) / (rate(ancora_run_success_total[5m]) + rate(ancora_run_failure_total[5m]))",
                    "legendFormat": "success_rate"
                }]
            },
            {
                "type": "timeseries",
                "title": "Run Latency P99",
                "targets": [{
                    "expr": "histogram_quantile(0.99, rate(ancora_run_latency_bucket[5m]))",
                    "legendFormat": "p99_ms"
                }]
            },
            {
                "type": "gauge",
                "title": "Error Budget Remaining",
                "targets": [{
                    "expr": "ancora_slo_budget_remaining",
                    "legendFormat": "budget_fraction"
                }]
            },
            {
                "type": "timeseries",
                "title": "Worker Utilization",
                "targets": [{
                    "expr": "ancora_worker_utilization",
                    "legendFormat": "utilization"
                }]
            },
            {
                "type": "timeseries",
                "title": "Queue Depth",
                "targets": [{
                    "expr": "ancora_queue_depth",
                    "legendFormat": "queued_runs"
                }]
            }
        ],
        "templating": {
            "list": [{
                "name": "tenant",
                "type": "query",
                "query": "label_values(ancora_run_success_total, tenant)"
            }]
        }
    })
}
