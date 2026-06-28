# Cost Analytics Guide

ancora-costan provides full-dimension cost analytics for the Ancora agent framework.

## Overview

The crate tracks spend across every relevant dimension and surfaces actionable
insights through anomaly detection, forecasting, and suggestions.

## Dimensions tracked

- Time series: per-event cost recording with hourly bucket aggregation
- By model: cost and token usage per LLM model
- By provider: cost and request count per LLM provider
- By tool: cost per tool invocation
- By tenant and project: multi-tenant cost allocation
- By capability: planner, reflection, routing, reasoning, generation, retrieval
- Cache savings: tracks cost avoided through prompt-cache hits

## Getting started

```rust
use ancora_costan::api::{CostAnalytics, CostEvent};
use ancora_costan::by_capability::Capability;

let mut analytics = CostAnalytics::new(3.0); // anomaly threshold = 3 sigma

let event = CostEvent {
    timestamp: 1_700_000_000,
    cost_usd: 0.05,
    tokens: 500,
    model: "claude-3-5-sonnet".to_string(),
    provider: "anthropic".to_string(),
    tool: None,
    tenant_id: "acme".to_string(),
    project_id: "chat-bot".to_string(),
    capability: Capability::Generation,
    cache_hit: false,
    full_cost_if_miss: 0.05,
    actual_cache_cost: 0.0,
};

if let Some(alert) = analytics.ingest(event) {
    eprintln!("Anomaly: {}", alert.description);
}

println!("Total cost: ${:.4}", analytics.total_cost());
```

## Dashboard

Call `analytics.snapshot("2025-01")` to obtain a `DashboardSnapshot`, then
`.to_json()` to render it as a JSON string suitable for an API response or log.

## Modules

| Module | Purpose |
|---|---|
| `timeseries` | Time-ordered cost recording and aggregation |
| `by_model` | Cost breakdown by LLM model |
| `by_provider` | Cost breakdown by provider |
| `by_tool` | Cost breakdown by tool invocation |
| `by_tenant` | Multi-tenant cost allocation |
| `by_capability` | Breakdown by agent capability type |
| `cache_savings` | Cache-hit savings measurement |
| `anomaly` | Z-score based anomaly detection |
| `forecast` | Linear regression and EMA forecasting |
| `suggestions` | Optimization suggestion engine |
| `dashboard` | Aggregated JSON dashboard |
| `api` | Primary public API surface |
