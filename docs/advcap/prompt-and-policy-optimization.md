# Prompt and Policy Optimization

Policy optimization in ancora is driven by evaluation feedback loops. The eval
harness (ancora-ageval) measures behavior quality; the baseline store tracks
regressions; and the report surfaces metrics to inform prompt iteration.

## Optimization Loop

1. Run the agent with the current prompt/policy
2. Evaluate output with the relevant `*Metric` functions
3. Compare against `BaselineStore` to detect regression
4. If regressed: iterate the prompt or policy
5. When passing: update the baseline with the new score

```rust
use ancora_ageval::{BaselineStore, PlanningMetric, MetricScore, EvalReport};

let mut store = BaselineStore::new(0.05);
store.set("planning_quality", 0.8); // initial baseline

// After a prompt change:
let score = PlanningMetric::score(&expected, &actual);
match store.check("planning_quality", score) {
    BaselineResult::Regressed { .. } => { /* revert prompt */ }
    BaselineResult::Passed { .. } => {
        store.set("planning_quality", score); // update baseline
    }
    BaselineResult::NoPrior => {}
}
```

## Policy-Level Optimization

For guardrail policies, measure catch rate and adjust patterns when rate drops:

```rust
let catch_rate = GuardrailMetric::score(triggered, total);
if catch_rate < 0.9 {
    // add more patterns to PiiInputGuardrail or InjectionInputGuardrail
}
```

## Tip

Always gate prompt/policy changes on a full eval run. A change that improves one
metric while regressing another should be rejected.
