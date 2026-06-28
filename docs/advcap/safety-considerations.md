# Safety Considerations

Ancora is built with safety as a first-class concern. This document covers the
threat model and the controls available.

## Threat Model

| Threat | Mitigation |
|---|---|
| Prompt injection in user input | `InjectionInputGuardrail` detects and blocks |
| PII leakage | `PiiInputGuardrail` redacts emails, SSN, DOB, CC, phone |
| Harmful output | `SafetyOutputGuardrail` blocks XSS, SQL drop, rm -rf, HARM markers |
| Schema violation | `SchemaOutputGuardrail` repairs or rejects malformed output |
| Unauthorized tool calls | `AllowDenyGuardrail` enforces allow/deny lists |
| Human approval bypass | `ApprovalGate` requires explicit `.approve()` before execution |
| Runaway cost | `TokenBudget` + `Throttle` + `Deadline` limit resource consumption |
| Reasoning errors | `ContradictionDetector` + `AbstentionPolicy` prevent false confidence |

## Guardrail Composition

Apply multiple guardrails in a `GuardrailPolicy` to get defense-in-depth:

```rust
let mut policy = GuardrailPolicy::new();
policy.add_input(InjectionInputGuardrail);
policy.add_input(PiiInputGuardrail);
policy.add_output(SafetyOutputGuardrail);
policy.add_output(SchemaOutputGuardrail);
```

## Audit Trail

Every guardrail decision is recorded in `GuardrailJournal`. Use it for post-hoc
review or compliance reporting:

```rust
let blocked = journal.blocked_count();
let repaired = journal.repaired_count();
```

## Sandbox Execution

`SandboxRunner::execute` runs synthesized tools in a sandboxed context where no
external I/O is possible. Do not bypass the sandbox by calling tool functions
directly without the `SandboxRunner` wrapper.

## Abstention

Agents should abstain rather than guess when confidence is low. Use `AbstentionPolicy`
to enforce this automatically:

```rust
let policy = AbstentionPolicy::new(0.7); // abstain if mean score < 0.7
```

## Human-in-the-loop

For high-stakes actions, require human approval before execution:

```rust
let mut gate = ApprovalGate::default();
gate.approve("send-email");
gate.is_approved("send-email"); // true only after explicit approval
```
