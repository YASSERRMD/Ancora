# Ancora Metrics Catalog

## Counters

| Metric | Labels | Description |
|--------|--------|-------------|
| `ancora_run_success_total` | `tenant` | Total successful agent runs |
| `ancora_run_failure_total` | `tenant` | Total failed agent runs |

## Histograms

| Metric | Buckets (ms) | Description |
|--------|-------------|-------------|
| `ancora_run_latency` | 10, 50, 100, 500, 1000 | End-to-end run latency |
| `ancora_step_latency` | 5, 25, 100, 500 | Per-step tool call latency |
| `ancora_journal_append_latency` | 1, 5, 10, 25, 50, 100, 250, 500, 1000 | Journal append latency |

## Gauges

| Metric | Labels | Description |
|--------|--------|-------------|
| `ancora_queue_depth` | `tenant` | Current queued run count |
| `ancora_worker_utilization` | (none) | Fraction of workers busy (0.0-1.0) |
| `ancora_slo_budget_remaining` | `slo` | Error budget remaining fraction |

## Provider metrics

| Metric | Labels | Description |
|--------|--------|-------------|
| `ancora_provider_error_rate` | `provider` | LLM provider error fraction |
| `ancora_tenant_cost_usd_total` | `tenant` | Cumulative cost in USD |

## Prometheus scrape configuration

```yaml
scrape_configs:
  - job_name: ancora
    static_configs:
      - targets: ['ancora-controlplane:8080']
    metrics_path: /metrics
    scrape_interval: 15s
```
