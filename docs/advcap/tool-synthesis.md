# Tool Synthesis

ancora-toolsynth generates and manages dynamically synthesized tools from
natural-language goals. It enforces safety through layered controls: schema
validation, sandbox execution, permission scoping, and an approval gate.

## Synthesizing a Tool

```rust
use ancora_toolsynth::{spec_from_goal, SynthRegistry, ApprovalGate, SandboxRunner};

let spec = spec_from_goal("search documents");
// spec.name = "search_documents", spec.effect_class = EffectClass::ReadOnly

let mut registry = SynthRegistry::default();
registry.register(spec.clone());

let mut gate = ApprovalGate::default();
gate.approve(&spec.name);

let result = SandboxRunner::execute(&spec, &serde_json::json!({}));
assert!(result.is_ok());
```

## Effect Classes

| Class | Sandbox allowed |
|---|---|
| `ReadOnly` | Yes |
| `WriteLocal` | Yes |
| `WriteExternal` | No |
| `Destructive` | No |

## Audit Trail

Every synthesis, approval, and execution is recorded in `SynthAudit`:

```rust
audit.record(tick, AuditEvent::Synthesized { tool_name: name.clone(), goal });
audit.record(tick, AuditEvent::Approved { tool_name: name, approver: "admin".into() });
```

## Integration with Guardrails

Combine `ApprovalGate` with `AllowDenyGuardrail` to ensure only approved tools
are callable at runtime:

```rust
policy.add_action(AllowDenyGuardrail::allow_only(vec![approved_tool.as_str()]));
```
