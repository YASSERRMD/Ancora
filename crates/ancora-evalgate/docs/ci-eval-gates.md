# Wiring Eval Gates into CI

The eval gate runs as a step in the PR CI pipeline.

## GitHub Actions Example

```yaml
name: Eval Gates

on:
  pull_request:
    branches: [main]

jobs:
  eval-gate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0

      - name: Build ancora-evalgate
        run: cargo build -p ancora-evalgate --release

      - name: Run eval gate checks
        run: cargo test -p ancora-evalgate

      - name: Check cost gate
        run: |
          cargo run --example cost_gate_check -- \
            --baseline baselines/cost.json \
            --candidate results/cost.json

      - name: Check latency gate
        run: |
          cargo run --example latency_gate_check -- \
            --baseline baselines/latency.json \
            --candidate results/latency.json
```

## Gate Failure Behaviour

When the gate fails, the CI step exits with a non-zero status code,
blocking the PR from being merged. The gate report is posted as a PR
comment containing a table of all checked metrics.

## Skipping Gates

Gates can be skipped by adding the label `skip-eval-gate` to the PR.
This is intended only for documentation-only or infrastructure PRs
that cannot affect model quality.
