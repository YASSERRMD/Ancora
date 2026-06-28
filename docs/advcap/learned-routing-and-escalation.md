# Learned Routing and Escalation

ancora-ageval's `RoutingMetric` quantifies how well a routing decision balances
output quality against resource cost.

## Routing Score

```rust
use ancora_ageval::RoutingMetric;

// quality = 0.9, cost = 20 tokens out of 100 max
let score = RoutingMetric::score(0.9, 20, 100);
// score = (0.9 + 0.8) / 2 = 0.85
```

## Escalation Pattern

Escalation routes tasks to a more capable (but costlier) handler when a cheaper
handler fails to meet a quality threshold:

```rust
let fast_quality = 0.6;
let fast_score = RoutingMetric::score(fast_quality, 5, 100);

if fast_score < 0.7 {
    // escalate: route to the more capable handler
    let full_quality = 0.95;
    let full_score = RoutingMetric::score(full_quality, 80, 100);
}
```

## Tracking Over Time

Use `BaselineStore` to detect when routing decisions degrade:

```rust
store.set("routing_cost_quality", 0.8);
// On next eval:
store.check("routing_cost_quality", current_score); // => Passed or Regressed
```

## Determinism

`RoutingMetric::score` is a pure arithmetic function. Given the same inputs it
produces the same output on any platform, making routing decisions fully reproducible.
