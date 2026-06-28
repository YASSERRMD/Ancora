# Advanced Feature Parity Matrix

This matrix shows which advanced capabilities are available and tested in each
Ancora language port, along with canonical numeric values for validation.

## Canonical Values

All language ports must produce these exact values for the 7 advanced metrics:

| Metric | Input | Expected |
|---|---|---|
| PlanningMetric | expected=4, matched=3 | 0.75 |
| ReflectionMetric | grew | 1.0 |
| ReflectionMetric | shrunk but changed | 0.5 |
| ReflectionMetric | unchanged | 0.0 |
| RoutingMetric | quality=0.9, cost=300, max=1000 | 0.8 |
| RoutingMetric | quality=0.85, cost=0, max=1000 | 0.925 |
| CoordinationMetric | assigned=3, completed=3 | 1.0 |
| GuardrailMetric | triggered=1, total=2 | 0.5 |
| ReasoningMetric | verified=4, total=5 | 0.8 |
| MemoryMetric | retained=9, total=10 | 0.9 |

## Language Coverage

| Capability | Rust | Go | Python | TypeScript | .NET | Java |
|---|---|---|---|---|---|---|
| Planning | yes | yes | reference | reference | reference | reference |
| Reflection | yes | yes | reference | reference | reference | reference |
| Routing | yes | yes | reference | reference | reference | reference |
| Coordination | yes | partial | - | - | - | - |
| Guardrails | yes | partial | - | - | - | - |
| Reasoning | yes | partial | - | - | - | - |
| Memory | yes | yes | reference | reference | reference | reference |
| Long-horizon | yes | - | - | - | - | - |
| Tool synthesis | yes | - | - | - | - | - |
| Skills | yes | - | - | - | - | - |

**reference** = language uses canonical constants from `ts_dotnet_java_batch.rs`

## Validation

Run `cargo test -p ancora-advpar` to validate all Rust values.
Run `go run ./examples/advanced-parity/` in `sdk/go/` to validate Go values.

All values must match within floating-point epsilon (1e-9).
