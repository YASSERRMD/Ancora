# Guardrails

ancora-guard provides composable input, output, and action guardrails for agent
pipelines. All decisions are journaled for audit and replay.

## Built-in Guardrails

| Guardrail | Type | Action |
|---|---|---|
| `PiiInputGuardrail` | Input | Repair: redact emails, SSN, DOB, credit card, phone |
| `InjectionInputGuardrail` | Input | Block: injection and jailbreak patterns |
| `SafetyOutputGuardrail` | Output | Block: XSS, SQL drop, rm -rf, HARM markers |
| `SchemaOutputGuardrail` | Output | Repair: wrap non-JSON output in `{}` |
| `AllowDenyGuardrail` | Action | Block: tools on denylist or not on allowlist |

## Policy Composition

```rust
use ancora_guard::{GuardrailPolicy, PiiInputGuardrail, SafetyOutputGuardrail, GuardrailJournal};

let mut policy = GuardrailPolicy::new();
policy.add_input(PiiInputGuardrail);
policy.add_output(SafetyOutputGuardrail);

let mut journal = GuardrailJournal::default();
let outcome = policy.check_input("user@example.com text", &mut journal, tick);
```

## Custom Guardrails

```rust
use ancora_guard::CustomInputGuardrail;

let g = CustomInputGuardrail::new("no_empty", |input| {
    if input.trim().is_empty() {
        GuardrailOutcome::Block("empty input".into())
    } else {
        GuardrailOutcome::Pass
    }
});
```

## Outcomes

| Outcome | Meaning |
|---|---|
| `Pass` | Input/output/action is safe |
| `Block(reason)` | Reject and return the reason |
| `Repair(fixed)` | Replace with the sanitized version |

## Journal

`GuardrailJournal::blocked_count()` and `repaired_count()` give aggregate stats.
All decisions are recorded in insertion order for deterministic replay.
