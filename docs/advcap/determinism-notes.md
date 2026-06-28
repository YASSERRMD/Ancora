# Determinism Notes

Ancora is designed for deterministic replay across all advanced capabilities.
Every state-changing operation is journaled with a monotonic u64 tick, not wall-clock time.

## Tick-based Time

All timestamps are abstract u64 ticks. This means:
- No dependency on system clocks
- Replay produces identical results when ticks are identical
- Tests can use any u64 as a tick (commonly 0, 1, or a counter)

```rust
let tick: u64 = 42;
let mut run = BackgroundRun::new("r", tick);
```

## Journal Replay

Every crate with stateful operations exposes a journal with replay semantics:

| Crate | Journal | Replay |
|---|---|---|
| ancora-orchestrate | `OrchestrateJournal` | `replay()` |
| ancora-coord | `CoordJournal` | `replay()` |
| ancora-guard | `GuardrailJournal` | `blocked_count/repaired_count` |
| ancora-reason | `ReasoningJournal` | `replay()` |
| ancora-ageval | `EvalReport` | deterministic `mean_score` |

## Idempotency

`BackgroundRun::apply_effect` is idempotent: applying the same effect string twice
records it only once:

```rust
run.apply_effect("step-1");
run.apply_effect("step-1"); // no-op
assert_eq!(run.effects_applied(), &["step-1"]);
```

## Avoiding Non-determinism

- Never use `std::time::Instant` or `SystemTime` inside capability code
- Never use `rand` without a seeded RNG; prefer deterministic inputs
- Do not use `HashMap` iteration order for result ordering; sort before emitting
- All journal entries must be appended in tick order

## Cross-language Parity

`ancora-adv-integration/src/tests/test_parity.rs` defines canonical numeric results
for each metric. When porting to another language, use these values as ground truth.

| Metric | Canonical Value |
|---|---|
| planning (3 of 4 matched) | 0.75 |
| reflection (grew) | 1.0 |
| routing (quality=0.9, cost=300, max=1000) | 0.85 |
| coordination (3 of 3) | 1.0 |
| guardrail (1 of 2) | 0.5 |
| reasoning (4 of 5) | 0.8 |
| memory (9 of 10) | 0.9 |
