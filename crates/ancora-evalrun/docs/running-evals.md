# Running Evals

This guide explains how to run evaluations using `ancora-evalrun`.

## Overview

An eval run executes a suite of test cases against a model or agent, collecting pass/fail outcomes, latency, and cost metrics.

## Basic Usage

Define your eval suite as a list of `EvalCase` structs, each with an `id`, `input`, and `expected` output. Then configure a `RunConfig` with a scorer function and seed, and use `Executor` and `RolloutRunner` to run the suite.

```rust
use ancora_evalrun::executor::{EvalCase, Executor, RunConfig, RunId, exact_match};
use ancora_evalrun::rollout::RolloutRunner;

let cases = vec![
    EvalCase { id: "q1".into(), input: "What is 2+2?".into(), expected: "4".into() },
];

let config = RunConfig {
    run_id: RunId("my-run".into()),
    scorer: exact_match,
    seed: 42,
};

let executor = Executor::new(config);
let runner = RolloutRunner::new(5); // 5 rollouts per case
let rollouts = runner.rollout_suite(&executor, &cases, &my_infer_fn);
```

## N-Rollouts

Running N rollouts per case enables statistical metrics (pass@k, confidence intervals). Use `RolloutRunner::new(n)` with the desired rollout count.

## Scorers

Two built-in scorers are provided:
- `exact_match`: requires trimmed exact equality
- `prefix_match`: requires actual output starts with expected

Custom scorers are any `fn(&str, &str) -> bool`.

## Seeding

Each rollout within a run gets a unique seed derived from the base seed plus the rollout index. This ensures reproducibility: the same base seed always produces the same sequence of rollout seeds.

## CLI

Use the CLI interface to run evals from a script:

```
run <suite-name> [--rollouts N] [--seed S] [--format json|html|text]
compare <run-id-a> <run-id-b>
list
```

## Output Formats

- `text` - human-readable summary
- `json` - machine-readable JSON with all metrics and per-case breakdown
- `html` - standalone HTML report with a sortable table
