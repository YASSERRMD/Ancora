# Eval Library Reference Link

The offline eval computation library is `ancora-evallib`.

## Location

Crate: `crates/ancora-evallib`
Documentation: `crates/ancora-evallib/README.md`

## Key APIs

- `EvalRunner::new()` - construct an offline eval runner
- `EvalRunner::run(input)` - execute an eval without network calls
- `EvalResult` - structured result with score, explanation, metadata
- `EvalMetric` - pluggable metric implementations (rouge, cosine, etc.)

## Usage in CI

The eval library is used by `ancora-evalgate` for CI gating.
See the [eval gate docs](../../../crates/ancora-evalgate/README.md).

## Integration with Quickstarts

The per-language quickstarts reference the eval library for recording scores.
See [quickstarts.md](quickstarts.md) for code examples.
