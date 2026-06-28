# Guardrails Guide

ancora-guard provides composable input, output, and action guardrails that can
be attached to an agent as a `GuardrailPolicy`.

## Guardrail Types

| Type | Interface | Checks |
|---|---|---|
| Input | `InputGuardrail` | What enters the agent |
| Output | `OutputGuardrail` | What the agent produces |
| Action | `ActionGuardrail` | What tools the agent may call |

## Built-in Guardrails

- `PiiInputGuardrail`: repairs inputs containing email, SSN, DOB, credit card, or phone markers
- `SafetyOutputGuardrail`: blocks outputs with XSS, SQL drop, rm -rf, or HARM markers
- `SchemaOutputGuardrail`: repairs outputs that are not valid JSON objects
- `InjectionInputGuardrail`: blocks jailbreak and system-prompt injection patterns
- `AllowDenyGuardrail`: blocks tools on a denylist or not on an allowlist

## Custom Guardrails

```rust
let g = CustomInputGuardrail::new("no_empty", |input| {
    if input.trim().is_empty() {
        GuardrailOutcome::Block("empty input".into())
    } else {
        GuardrailOutcome::Pass
    }
});
```

## Policy Composition

```rust
let mut policy = GuardrailPolicy::new();
policy.add_input(PiiInputGuardrail);
policy.add_output(SafetyOutputGuardrail);
policy.add_action(AllowDenyGuardrail::deny(vec!["drop_table"]));
```

All decisions are journaled in `GuardrailJournal` for audit and replay.
