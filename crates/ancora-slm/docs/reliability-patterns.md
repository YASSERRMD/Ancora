# Reliability Patterns for Small Language Models

Small models fail more often than large ones. This document describes the reliability patterns available in `ancora-slm` and when to apply each.

## Pattern Catalogue

### Pattern 1: Retry with Stronger Constraints

On failure, re-prompt with an increasingly explicit output format instruction.

**When to use**: The model understands the task but produces slightly malformed output.

**Implementation**: `constrained::run_constrained` — retries up to `max_retries` times, each time adding a stronger JSON-only instruction.

**Cost**: O(retries) inference calls. Set `max_retries` to 2-3.

### Pattern 2: Output Verification Gate

Run every model output through a pipeline of verifiers before accepting it.

**When to use**: Always — even for simple tasks.

**Implementation**: `verifier::run_verifiers` with a chain of `Verifier` implementations.

**Built-in verifiers**:
- `NonEmptyVerifier` — catches empty outputs (common hallucination pattern).
- `ValidJsonVerifier` — catches syntactically broken JSON.
- `RequiredKeysVerifier` — catches structurally incomplete JSON objects.
- `ContainsKeywordsVerifier` — catches off-topic responses.
- `LengthVerifier` — catches truncated or over-verbose outputs.

### Pattern 3: Tool-Call Repair

Automatically fix common tool-call format errors before dispatch.

**When to use**: Whenever the SLM is expected to produce tool calls.

**Implementation**: `repair::repair_tool_call`.

**Repairs applied**:
1. Extract JSON from surrounding prose.
2. Fix trailing commas.
3. Convert single-quoted strings to double-quoted.
4. Normalise field names (`function_name` → `name`, `args` → `arguments`).

### Pattern 4: Step Decomposition

Split a complex task into a sequence of smaller, independently verifiable steps.

**When to use**: Multi-step reasoning tasks where the full task exceeds the model's reliable capability.

**Benefits**:
- Each step can be independently verified.
- Failure is localised to the specific step.
- Prior step outputs provide context for subsequent steps.
- Optional steps let the pipeline continue past non-critical failures.

**Implementation**: `decompose::DecompositionPlan` + `decompose::execute_plan`.

### Pattern 5: Few-Shot Priming

Prepend 2-5 high-quality examples to anchor the model's output format.

**When to use**: When the model has seen the format in training but needs a reminder, or when zero-shot performance is inconsistent.

**Implementation**: `fewshot::inject_few_shots`.

**Tips**:
- 2-3 examples usually suffice; more can confuse small models by filling the context window.
- Sort examples by quality (descending) — the best example is shown first.
- Use examples from the same domain as the task.

### Pattern 6: Model Escalation

When all SLM retry attempts fail, escalate to a larger (but slower/costlier) model.

**When to use**: As a last-resort fallback, not the primary path.

**Implementation**: `escalate::run_with_escalation`.

**Policy configuration**:
```rust
EscalationPolicy {
    max_slm_attempts: 2,           // try SLM twice before escalating
    escalation_tier: ModelTier::Large,
}
```

## Combining Patterns

Patterns compose naturally. A production pipeline for a tool-calling agent might look like:

```
User request
   → Few-shot injection (fewshot)
   → Prompt formatting (prompt)
   → SLM inference
   → Tool-call repair (repair)
   → Verifier gate (verifier)
   → Constrained retry if needed (constrained)
   → Escalation if budget exhausted (escalate)
```

## Determinism and Testability

All patterns accept plain `Fn(&str) -> String` model functions, so they are:

- **Unit-testable offline** — swap in a `make_replay_fn` stub.
- **Deterministic in CI** — same inputs always produce the same outputs.
- **Replayable** — record real model calls with `ReplayStore`, replay them later.

## Failure Mode Reference

| Failure mode | Recommended pattern |
|---|---|
| Empty output | `NonEmptyVerifier` + retry |
| Broken JSON | `ValidJsonVerifier` + `run_constrained` |
| Missing keys | `RequiredKeysVerifier` + retry |
| Malformed tool call | `repair_tool_call` |
| Off-topic response | `ContainsKeywordsVerifier` + few-shot |
| Task too complex | `DecompositionPlan` |
| Repeated failure | `EscalationPolicy` |
