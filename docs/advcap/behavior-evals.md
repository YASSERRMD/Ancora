# Behavior Evals

ancora-ageval provides quantitative metrics for evaluating advanced agent behaviors,
along with regression baseline storage and eval report generation.

## Metric Reference

| Metric | Function | Returns |
|---|---|---|
| Planning quality | `PlanningMetric::score(expected, actual)` | matched / expected |
| Reflection improvement | `ReflectionMetric::score(before, after)` | 0.0 / 0.5 / 1.0 |
| Routing cost-quality | `RoutingMetric::score(quality, cost, max_cost)` | blended score |
| Coordination success | `CoordinationMetric::score(assigned, completed)` | completed / assigned |
| Guardrail catch rate | `GuardrailMetric::score(triggered, total)` | triggered / total |
| Reasoning correctness | `ReasoningMetric::score(verified, total)` | verified / total |
| Memory retention | `MemoryMetric::score(retained, total)` | retained / total |

## Eval Report

```rust
use ancora_ageval::{EvalReport, MetricScore};

let mut report = EvalReport::new("run-42", tick);
report.add_score(MetricScore::new("planning_quality", 0.9));
report.add_score(MetricScore::new("reasoning_correctness", 0.85));
println!("{}", report.summary()); // EvalReport[run-42] tick=1 metrics=2 mean=0.875 regressions=0
```

## Regression Detection

```rust
use ancora_ageval::{BaselineStore, BaselineResult};

let mut store = BaselineStore::new(0.05); // 5% tolerance
store.set("planning_quality", 0.9);

match store.check("planning_quality", 0.8) {
    BaselineResult::Regressed { delta, .. } => eprintln!("Regression: {}", delta),
    BaselineResult::Passed { .. } => {},
    BaselineResult::NoPrior => {},
}
```

## Dataset

Use `EvalDataset` and `EvalSample` to organize eval fixtures by tag:

```rust
let planning_samples = dataset.by_tag("planning");
```
