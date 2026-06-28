# Patterns and Anti-patterns

## Patterns

### Journal Everything

Record all decisions in the appropriate typed journal. This enables:
- Deterministic replay
- Audit compliance
- Regression detection across runs

### Score Before You Ship

Run `EvalReport` after every pipeline execution. Compare against `BaselineStore`
to catch regressions before they reach production.

### Guardrails at Every Boundary

Attach `GuardrailPolicy` at the entry and exit of every agent boundary:
- Input: block injection, redact PII
- Output: block unsafe content, repair schema
- Action: allowlist only approved tool names

### Depth Limit Orchestration

Always wrap recursive fan-out with `DepthLimiter` to prevent unbounded recursion:

```rust
let mut limiter = DepthLimiter::new(5);
limiter.enter()?; // returns Err if max depth exceeded
// ... recursive work ...
limiter.exit();
```

### Abstain Over Hallucinate

Use `AbstentionPolicy` when evidence confidence is low. Abstaining is always
preferable to returning a low-confidence claim as fact.

## Anti-patterns

### Sharing State Without Roles

Never write to a `Blackboard` key without first calling `claim_role`. Unclaimed
keys can be overwritten by any agent, leading to races.

### Skipping Guardrails on Internal Calls

Guardrails are not just for user input. Apply them to any text that crosses an
agent boundary, including agent-to-agent messages and tool outputs.

### Mutable Journals

Journals are append-only by design. Never remove or modify journal entries.
Replay correctness depends on immutability.

### Network Calls in Metrics

All eval metrics (`PlanningMetric`, `ReasoningMetric`, etc.) are pure functions
over in-memory data. Never add network I/O to metric computation. Pre-fetch any
external data before calling the metric function.

### Skipping Baseline

Running evals without storing a baseline means regressions go undetected.
Always `store.set(metric_name, score)` after a known-good run.
