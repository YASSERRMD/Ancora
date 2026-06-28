# Determinism Guarantees for Advanced Features

All Ancora advanced capabilities are designed to be fully deterministic.
Given identical inputs, every capability produces identical outputs on every run.

## Design Principles

- Abstract monotonic u64 ticks replace wall-clock time everywhere
- No `std::time::Instant` or `SystemTime` inside capability code
- No thread-local RNG; all computations are pure functions of their inputs
- All journals are append-only Vec structures; no hash-map iteration order in output
- Idempotent effects: `BackgroundRun::apply_effect` is a no-op for duplicates

## Journal Replay Guarantees

| Crate | Journal Type | Replay Method |
|---|---|---|
| ancora-orchestrate | (ResultAggregator) | `.merge_outputs()` |
| ancora-coord | CoordJournal | `.replay()` |
| ancora-guard | GuardrailJournal | `.decisions()` |
| ancora-reason | ReasoningJournal | `.replay()` |
| ancora-memcon | ConsolidationJournal | `.replay_events()` |
| ancora-skills | SkillJournal | `.replay()` |
| ancora-toolsynth | SynthAudit | `.entries()` |

## Metric Stability

All ageval metrics are pure functions:

```
PlanningMetric::score(expected, actual)   -> matched / expected.len()
ReflectionMetric::score(before, after)    -> 0.0 / 0.5 / 1.0
RoutingMetric::score(quality, cost, max)  -> (quality + (1 - cost/max)) / 2
CoordinationMetric::score(assigned, done) -> done / assigned
GuardrailMetric::score(triggered, total)  -> triggered / total
ReasoningMetric::score(verified, total)   -> verified / total
MemoryMetric::score(retained, total)      -> retained / total
```

## Cross-language Canonical Values

See `docs/advcap/determinism-notes.md` for the canonical numeric table that
all language ports must match.

## Test Coverage

The `ancora-advdet` crate contains 67 tests organized into 17 modules,
covering every advanced crate's determinism, replay, and parity properties.
