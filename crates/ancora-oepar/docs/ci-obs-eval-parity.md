# CI: Observability and Eval Parity Suite

## Workflow

The parity suite runs on every PR and push to main via `.github/workflows/ancora-oepar.yml`.

## Steps

1. `cargo test -p ancora-oepar` - runs all 56 unit tests offline.
2. Checks trace count equality across all six language reference traces.
3. Checks eval score equality within a 0.01 tolerance.
4. Checks cost attribute completeness and value equality.
5. Checks polyglot trace parent-link validity.

## Pass Criteria

- All tests green.
- No parity issue vectors returned by `check_*_parity` helpers.
- No PII detected in redacted outputs.

## Failure Handling

On failure, the CI job outputs a summary of all parity issues to stderr. A fix branch named `fix-235-<slug>` must be opened; force-pushing to main is not permitted.
