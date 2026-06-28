# Regression Gating Guide

ancora-evalgate blocks CI when a PR causes a statistically significant
regression in quality, cost, or latency metrics.

## How It Works

1. **Baseline** - each eval dataset has a stored set of accepted metric values.
2. **Candidate run** - the PR triggers eval runs that produce new metric values.
3. **Threshold policy** - each metric has an allowed regression (absolute or relative).
4. **Significance check** - only regressions that exceed the significance threshold
   (default alpha=0.05) are treated as real.
5. **Gate decision** - if any metric is both beyond the threshold AND statistically
   significant, the gate fails and blocks the PR.

## Configuring Thresholds

```rust
use ancora_evalgate::threshold::{MetricDirection, ThresholdKind, ThresholdPolicy, ThresholdRegistry};

let mut registry = ThresholdRegistry::new();
registry.register(ThresholdPolicy::new(
    "accuracy",
    MetricDirection::HigherIsBetter,
    ThresholdKind::Absolute(0.02),  // allow up to 2 pp drop
));
registry.register(ThresholdPolicy::new(
    "cost_usd",
    MetricDirection::LowerIsBetter,
    ThresholdKind::Relative(0.10),  // allow up to 10% cost increase
));
```

## Flaky Evals

For noisy evals, configure retry behaviour:

```rust
use ancora_evalgate::flaky::{FlakyPolicy, evaluate_with_retry};

let policy = FlakyPolicy::new(2, 0.5); // 2 retries, block only if >50% regress
```

## Cost and Latency Gates

Dedicated gates are available for cost and latency:

```rust
use ancora_evalgate::cost_gate::{CostGateConfig, blocks as cost_blocks};
use ancora_evalgate::latency_gate::{LatencyGateConfig, Percentile, any_blocks};

let cost_cfg = CostGateConfig { max_relative_increase: 0.10 };
if cost_blocks(baseline_cost, candidate_cost, &cost_cfg) {
    eprintln!("cost regression detected");
}

let lat_cfg = LatencyGateConfig {
    max_relative_increase: 0.20,
    percentiles: vec![Percentile::P50, Percentile::P95],
};
if any_blocks(&baseline_ms, &candidate_ms, &lat_cfg) {
    eprintln!("latency regression detected");
}
```
