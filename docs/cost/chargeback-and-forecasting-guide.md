# Chargeback and Forecasting Guide

## Chargeback reports

Generate a report for a billing period:

```rust
use ancora_cost::ChargebackReport;

let report = ChargebackReport::generate(&attributor, period_start, period_end);
for line in &report.lines {
    println!("{}: ${:.4} ({} tokens)", line.tenant_id, line.total_cost_usd, line.total_tokens);
}
println!("Total: ${:.4}", report.total_cost_usd());
```

Reports are sorted by tenant_id for deterministic output. Export to JSON with `serde_json::to_string`.

## Cost API

Get current budget status for a tenant:

```rust
use ancora_cost::TenantCostSummary;

let summary = TenantCostSummary::from_budget(&budget);
// summary.spent_usd, summary.remaining_usd, summary.pct_used
```

## Cost dashboard

Generate a JSON snapshot for display:

```rust
use ancora_cost::cost_dashboard;

let dashboard = cost_dashboard(&attributor);
// Returns: { title, total_cost_usd, total_tokens, record_count }
```

## Forecasting methodology

The forecaster uses the arithmetic mean of recent daily cost samples to predict future spend:

```
forecast(days) = mean(daily_samples) * days
```

This is suitable for workloads with relatively stable usage. For bursty workloads, consider using the 90th percentile of samples rather than the mean.

## Cheaper model suggestion

When the 30-day forecast exceeds the monthly budget, the system suggests `gpt-4o-mini` as a lower-cost alternative. Override this logic by implementing your own model selection policy.

## Integration with billing

Export chargeback lines to your billing system at end of period. Recommended flow:

1. At period end, generate `ChargebackReport`.
2. Export to billing system via API or CSV.
3. Call `budget.rollover(now)` to reset the period.
4. Archive old `CostRecord`s to cold storage.
