# Behavior Eval Guide

ancora-ageval provides quantitative metrics for evaluating advanced agent behaviors.

## Available Metrics

| Metric | Module | Measures |
|---|---|---|
| `PlanningMetric` | `planning` | Fraction of expected plan steps present in output |
| `ReflectionMetric` | `reflection` | Whether self-critique improved the output |
| `RoutingMetric` | `routing` | Cost-quality trade-off for routing decisions |
| `CoordinationMetric` | `coordination` | Fraction of assigned agents that completed |
| `GuardrailMetric` | `guardrail_metric` | Fraction of unsafe inputs caught |
| `ReasoningMetric` | `reasoning_metric` | Fraction of reasoning steps verified |
| `MemoryMetric` | `memory_metric` | Fraction of important items retained |

## Running an Eval

```rust
use ancora_ageval::{PlanningMetric, EvalReport, MetricScore, BaselineStore};

let score = PlanningMetric::score(&expected_steps, &actual_steps);

let mut report = EvalReport::new("my-run", tick);
report.add_score(MetricScore::new("planning_quality", score));

let mut store = BaselineStore::new(0.05); // 5% tolerance
store.set("planning_quality", 0.9);

if let ancora_ageval::BaselineResult::Regressed { .. } = store.check("planning_quality", score) {
    report.add_regression("planning_quality");
}

println!("{}", report.summary());
```

## Regression Detection

`BaselineStore` compares a metric score against a stored baseline. A regression
is declared when the score drops by more than `tolerance` below the baseline:

```rust
let mut store = BaselineStore::new(0.05); // 5% tolerance
store.set("reasoning_correctness", 0.9);
// Score of 0.83 is 0.07 below baseline -> regression
store.check("reasoning_correctness", 0.83); // => Regressed
```
