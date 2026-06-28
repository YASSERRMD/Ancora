# CI: Sample App Parity

## What CI checks

The CI pipeline for ancora-appspar runs:

```
cargo build -p ancora-appspar
cargo test -p ancora-appspar
```

This covers:
- All six language modules compile cleanly
- All 10 test files pass (go, python, ts, dotnet, java, rust, parity, equal_traces, guardrails, polyglot_a2a)
- No network calls are made (all tests run offline)

## Test count

The test suite contains tests across 10 files. All tests are `#[test]`
functions that run without any external services.

## Adding a new language

1. Add a new module file under `src/`.
2. Declare it as `pub mod` in `src/lib.rs`.
3. Implement `feature_list()` returning all `REQUIRED_FEATURES`.
4. Add a test file under `src/tests/`.
5. Register the module in `parity::run_all()`.
