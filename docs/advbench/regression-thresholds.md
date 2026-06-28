# Regression Thresholds

Regression thresholds are defined in `crates/ancora-advbench/src/thresholds.rs`
as the `BASELINE` constant.

## Gate logic

CI runs `cargo test -p ancora-advbench` and the `regression_gate` test asserts:

```
elapsed_ns < baseline_ns * 2
```

for every capability.  A 2x multiplier absorbs machine variance, parallel CI
jobs, and cold-cache effects.

## Current baseline values

| Capability | baseline_ns | CI limit (2x) |
|---|---|---|
| planner_ns | 200,000,000 | 400,000,000 |
| reflection_ns | 10,000,000 | 20,000,000 |
| routing_ns | 10,000,000 | 20,000,000 |
| optimization_ns | 200,000,000 | 400,000,000 |
| memory_consolidation_ns | 50,000,000 | 100,000,000 |
| coordination_ns | 10,000,000 | 20,000,000 |
| guardrail_ns | 10,000,000 | 20,000,000 |
| reasoning_ns | 10,000,000 | 20,000,000 |
| lh_checkpoint_ns | 20,000,000 | 40,000,000 |
| skills_jit_ns | 20,000,000 | 40,000,000 |

## Updating thresholds

If a legitimate performance regression is identified, update `BASELINE` in
`thresholds.rs` and include the following in the PR description:

1. Before/after benchmark output (`cargo test -p ancora-advbench -- --nocapture`)
2. Profiling evidence of what changed
3. Justification that the regression is acceptable

Never raise a threshold without evidence.

## How to run locally

```bash
cargo test -p ancora-advbench
```

For verbose timing output:

```bash
cargo test -p ancora-advbench -- --nocapture 2>&1 | grep elapsed
```
