# Composing Advanced Capabilities

ancora's advanced capability crates are designed to compose without coupling.
Each crate exposes a clean API and communicates through value types, not trait objects
or callbacks shared across crates.

## Recommended Composition Order

```
fan_out (orchestrate)
  -> check_input (guard)
  -> decompose + verify (reason)
  -> consolidate (memcon)
  -> score (ageval)
  -> report (ageval)
```

## Example: Orchestrate + Guard + Reason + Eval

```rust
use ancora_orchestrate::fan_out;
use ancora_guard::{GuardrailPolicy, InjectionInputGuardrail, GuardrailJournal};
use ancora_reason::{StepDecomposer, StepVerifier};
use ancora_ageval::{ReasoningMetric, EvalReport, MetricScore};

// 1. Fan out tasks
let tasks = fan_out("orch", "worker", vec![serde_json::json!("claim-A")], "root");

// 2. Guard inputs
let mut policy = GuardrailPolicy::new();
policy.add_input(InjectionInputGuardrail);
let mut journal = GuardrailJournal::default();
for task in &tasks {
    policy.check_input(task.input.as_str().unwrap_or(""), &mut journal, 1);
}

// 3. Reason about each claim
let claims = tasks.iter().map(|t| t.input.as_str().unwrap_or("").to_string()).collect();
let mut steps = StepDecomposer::decompose(claims);
let mut verified = 0;
for step in steps.iter_mut() {
    if StepVerifier::verify(step, |c| !c.is_empty()).passed { verified += 1; }
}

// 4. Eval
let mut report = EvalReport::new("pipeline", 1);
report.add_score(MetricScore::new("reasoning", ReasoningMetric::score(verified, steps.len())));
println!("{}", report.summary());
```

## Shared Journal Pattern

Each capability exposes its own typed journal. Collect journal entries from all
stages at the end of a run to build a complete audit trail.

## Determinism

All advanced crates use in-memory stores and u64 tick counters. No calls to
`std::time::Instant` or external APIs occur inside capability logic. Replaying
journals produces the same output as the original run.
