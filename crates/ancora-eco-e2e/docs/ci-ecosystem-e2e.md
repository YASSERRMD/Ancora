# CI: Ecosystem E2E

## Workflow

The ecosystem e2e tests run as part of the standard CI pipeline.

## Commands

```
cargo build -p ancora-eco-e2e
cargo test -p ancora-eco-e2e
```

## Requirements

- No network access required
- All tests are in-memory
- Tests must complete in under 2 minutes
- Zero external crate dependencies

## Failure Handling

If any test fails, fix on a dedicated branch and open a new PR.
Do not force-push to main.
