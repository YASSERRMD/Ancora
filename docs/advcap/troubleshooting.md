# Troubleshooting

## Compiler Errors

### `expected FnMut, found Fn`

Closures passed to `FactChecker::check` must be `FnMut` because the checker
calls them multiple times and may need to capture mutable state:

```rust
// Wrong: Fn
let fc = FactChecker::check("claim", |c| db.get(c));

// Right: explicitly captured mutable
let mut db = db;
let fc = FactChecker::check("claim", |c| db.get(c));
```

### `no method named new found for type Blackboard`

`Blackboard`, `SkillRegistry`, `SynthRegistry`, and `ApprovalGate` use
`#[derive(Default)]` for construction:

```rust
// Wrong
let b = Blackboard::new();

// Right
let b = Blackboard::default();
```

### `cannot find field index on AgentTask`

`AgentTask` has no `.index` field. Use a literal tick instead:

```rust
// Wrong
task.index as u64

// Right
1_u64
```

### `fan_out inputs must be Vec<serde_json::Value>`

`fan_out` expects `Vec<serde_json::Value>`, not `Vec<String>`:

```rust
use serde_json::json;

// Wrong
let inputs = vec!["step-1".to_string()];

// Right
let inputs = vec![json!("step-1"), json!("step-2")];
```

### `AllowDenyGuardrail::allow_only takes Vec<&str>`

Pass `&str` slices, not owned `String`:

```rust
// Wrong
guardrail.allow_only(vec!["search".to_string()]);

// Right
guardrail.allow_only(vec!["search"]);
```

## Test Failures

### Tests pass locally but fail in CI

Check that all crates are listed in the workspace `Cargo.toml`. A crate not in
the workspace is silently ignored by `cargo test --workspace`.

### Parity test fails

Verify you are using the canonical formulas in `determinism-notes.md`. All metrics
use integer numerator / integer denominator arithmetic. Floating-point intermediate
accumulation should be avoided.

### Guardrail journal counts do not match

Journal counts reflect decisions across the lifetime of a single `GuardrailJournal`
instance. If you create a new journal per request, counts will not accumulate.

## Performance

If `test_perf.rs` is slow, check:
- `fan_out` with 100 tasks is in-memory and should complete in under 50ms
- `ContradictionDetector::detect` is O(n^2); use fewer than 1000 steps per call
- `ReasoningJournal::events()` clones the event list; prefer `replay()` for read-only use
