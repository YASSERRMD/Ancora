# Baseline Management

Baselines are the accepted metric values against which PR runs are compared.

## Storage

`BaselineStore` holds all baselines in memory, keyed by dataset name.
For persistent storage, serialize the `Baseline` structs (e.g. to JSON or TOML)
and reload them at CI startup.

## Creating a Baseline

```rust
use ancora_evalgate::baseline::{Baseline, BaselineStore};

let mut store = BaselineStore::new();
let mut baseline = Baseline::new("mmlu");
baseline.set("accuracy", 0.85);
baseline.set("cost_usd", 0.42);
store.upsert(baseline);
```

## Updating After an Approved PR

When a PR improves a metric and is approved for merge, update the baseline
so future PRs are compared against the new accepted value:

```rust
use std::collections::HashMap;

let mut new_values = HashMap::new();
new_values.insert("accuracy".to_string(), 0.88);

if let Some(b) = store.get_mut("mmlu") {
    b.update_from(&new_values);
}
```

## Baseline Versioning

Store baselines in version control (e.g. a `baselines/` directory committed
to the repository). The CI pipeline loads the baseline for the base branch,
runs evals on the PR branch, then compares using ancora-evalgate.

After a successful merge, the CI pipeline writes the updated baseline back
to the `baselines/` directory and commits it to `main`.

## Missing Baselines

If no baseline exists for a dataset, the gate skips that dataset rather
than failing. This allows new datasets to be introduced without blocking CI.
