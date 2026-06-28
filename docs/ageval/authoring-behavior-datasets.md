# Authoring Behavior Datasets

`EvalDataset` and `EvalSample` provide a flexible format for organizing
behavior evaluation fixtures.

## Sample Structure

Each `EvalSample` has:
- `id`: unique string identifier
- `tags`: list of capability tags (used for filtering)
- `metadata`: key-value pairs for test context

```rust
let sample = EvalSample::new("plan-001")
    .with_tag("planning")
    .with_tag("search")
    .with_meta("goal", "web-search-summarize")
    .with_meta("version", "1.0");
```

## Dataset Organization

Group samples by capability tag, then use `by_tag()` to select them at eval time:

```rust
let planning_samples = dataset.by_tag("planning");
let routing_samples = dataset.by_tag("routing");
```

## Metric Input Conventions

| Metric | Expected inputs from metadata/fields |
|---|---|
| `PlanningMetric` | `expected_steps: Vec<String>`, `actual_steps: Vec<String>` |
| `ReflectionMetric` | `before: &str`, `after: &str` |
| `RoutingMetric` | `quality: f64`, `cost: u64`, `max_cost: u64` |
| `CoordinationMetric` | `assigned: usize`, `completed: usize` |
| `GuardrailMetric` | `triggered: usize`, `total: usize` |
| `ReasoningMetric` | `verified: usize`, `total: usize` |
| `MemoryMetric` | `retained: usize`, `total: usize` |

Store these values in `EvalSample::metadata` as strings and parse them at eval time.
