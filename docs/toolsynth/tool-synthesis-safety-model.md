# Tool Synthesis Safety Model

ancora-toolsynth enables agents to generate and register new tools at runtime
within a layered safety model: schema validation, sandbox execution,
permission scoping, human approval, and audit trail.

## Layers

1. Schema validation: every generated spec must pass `SchemaValidator::validate`
2. Sandbox: `SandboxRunner` allows only ReadOnly and WriteLocal effects
3. Permission scope: `PermissionScope` further restricts the effect classes allowed
4. Approval gate: `ApprovalGate` requires explicit human approval before execution
5. Audit trail: `SynthAudit` records every synthesis, approval, revocation, and execution

## Effect Classes

| Class | Description | Sandbox allowed |
|---|---|---|
| ReadOnly | No side effects | Yes |
| WriteLocal | In-process or local file writes | Yes |
| WriteExternal | Network or IPC calls | No |
| Destructive | Irreversible deletes or drops | No |

## Tool Reuse

`SynthCache` caches synthesized tools by goal string to avoid repeated synthesis
for equivalent requests. Cached tools are recorded with `AuditEvent::Cached`.
