# Cost Governance Guide

## Budget definitions

### Tenant budget

```rust
use ancora_cost::{TenantBudget, BudgetPeriod};

let budget = TenantBudget::new(
    "tenant-a",
    100.0,  // hard limit USD
    0.8,    // soft limit at 80%
    BudgetPeriod::Monthly,
    period_start_secs,
);
```

### Project budget

```rust
use ancora_cost::ProjectBudget;

let budget = ProjectBudget::new("proj-analytics", 500.0, 0.9);
```

## Enforcement

Call `record_spend(amount)` before dispatching each run step:

```rust
match budget.record_spend(step_cost) {
    Ok(()) => { /* continue */ }
    Err(CostError::SoftCapWarning { pct }) => {
        log::warn!("cost budget at {pct:.1}%, consider scaling back");
        // continue the run
    }
    Err(CostError::HardCapExceeded { budget, spent }) => {
        // stop the run immediately
        return Err(RunError::BudgetExceeded);
    }
}
```

## Period rollover

Call `rollover(now)` at the start of each period (daily/weekly/monthly):

```rust
budget.rollover(now_secs);
```

If the current time exceeds the period end, `spent_usd` resets to 0 and the period_start advances to `now`.

## Cost attribution

All run costs should be recorded to `CostAttributor` for reporting:

```rust
use ancora_cost::{CostAttributor, CostRecord};

let mut attributor = CostAttributor::default();
attributor.record(CostRecord {
    tenant_id: "tenant-a".into(),
    run_id: run_id.clone(),
    model: "gpt-4o".into(),
    provider: "openai".into(),
    tool: Some("web_search".into()),
    tokens: 1500,
    cost_usd: 0.03,
});
```

Query: `attributor.total_by_tenant("tenant-a")`, `total_by_model("gpt-4o")`, etc.

## Forecasting

```rust
use ancora_cost::CostForecaster;

let f = CostForecaster::new(last_7_days_costs);
let forecast_30d = f.forecast(30);
if let Some(model) = f.suggest_cheaper_model(forecast_30d, monthly_budget) {
    log::info!("Consider switching to {model}");
}
```
