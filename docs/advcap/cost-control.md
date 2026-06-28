# Cost Control Across Capabilities

All Ancora capabilities operate without external calls. "Cost" in this framework
refers to compute tokens tracked via `TokenBudget`, not billing costs.

## Token Budget

ancora-cost provides `TokenBudget` for bounded agent runs:

```rust
use ancora_cost::TokenBudget;

let mut budget = TokenBudget::new(1000);
budget.consume(50); // deduct 50 tokens
assert!(budget.remaining() == 950);
assert!(!budget.is_exhausted());
```

## Routing with Cost Awareness

`RoutingMetric` accepts a cost value alongside quality to blend both into a score:

```rust
use ancora_ageval::RoutingMetric;

// quality=0.9, cost=200, max_cost=1000 -> cost_efficiency=0.8 -> blended=0.85
let score = RoutingMetric::score(0.9, 200, 1000);
```

Prefer routes with high blended scores to balance quality and cost.

## Cost-bounded Eval Runs

Set a baseline tolerance wide enough to avoid false regressions when cost is high:

```rust
use ancora_ageval::BaselineStore;

let mut store = BaselineStore::new(0.1); // 10% tolerance for cost-sensitive runs
store.set("routing_efficiency", 0.85);
```

## Throttling in Long-horizon Agents

`Throttle` prevents runaway per-tick operation counts:

```rust
use ancora_lh::Throttle;

let mut throttle = Throttle::new(10); // max 10 ops per tick
for _ in 0..10 {
    throttle.try_op(tick)?;
}
throttle.try_op(tick).unwrap_err(); // Throttled
```

## Practical Limits

| Resource | Control |
|---|---|
| Token spend per run | `TokenBudget::consume` |
| Ops per tick | `Throttle::new(max_ops)` |
| Time per run | `Deadline::check(now)` |
| Routing cost | `RoutingMetric::score(quality, cost, max)` |
