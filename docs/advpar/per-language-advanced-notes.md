# Per-language Advanced Notes

## Rust (canonical)

All advanced capabilities are fully implemented in Rust. The `ancora-advpar` crate
contains 94 parity tests confirming canonical values. Use these as ground truth.

## Go

The Go SDK has a standalone advanced parity example:

```bash
cd sdk/go && go run ./examples/advanced-parity/
```

All 10 metric checks pass with output `ok <metric> = <value>`.
The Go implementation uses pure arithmetic without any external dependencies.

## Python

For Python ports, implement the 7 metric functions using the canonical formulas:

```python
def planning_score(expected, matched): return 1.0 if not expected else matched / len(expected)
def reflection_score(before, after):
    if before == after: return 0.0
    return 1.0 if len(after) > len(before) else 0.5
def routing_score(quality, cost, max_cost):
    if max_cost == 0: return quality
    return (quality + (1 - cost / max_cost)) / 2
```

Validate against: `PLANNING_3_OF_4 = 0.75`, `ROUTING_0_9_300 = 0.8`, etc.

## TypeScript

TypeScript port should use the same integer-division semantics as Go and Rust.
Note: use integer coercion (`Math.trunc`) for count operations to avoid float drift.

## .NET

Use `double` (64-bit) arithmetic. C# and Rust share IEEE 754 semantics, so
values will match exactly when using identical formulas.

## Java

`double` arithmetic matches. Avoid `Math.round` before comparison;
use `Math.abs(a - b) < 1e-9`.

## Validation Protocol

1. Run the canonical Rust tests: `cargo test -p ancora-advpar`
2. Implement each metric function in the target language
3. Assert all values match the `ts_dotnet_java_batch.rs` constants
4. Report any discrepancy > 1e-9 as a parity failure
