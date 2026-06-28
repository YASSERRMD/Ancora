# Limits and Guardrails

## Schema Guardrails

- Input schema must have a `type` field with a valid JSON Schema primitive.
- Schemas without `type` are rejected to prevent untyped tool invocations.

## Sandbox Guardrails

- WriteExternal and Destructive effects cannot run in the sandbox.
- Synthesized code never has access to network or filesystem by design.

## Approval Guardrails

- A tool is blocked by default; approval is explicit opt-in.
- Revocation takes effect immediately with no grace period.

## Audit Guardrails

- Every synthesis event is written to the immutable audit trail.
- The audit trail is append-only: entries cannot be deleted or modified.

## Cache Guardrails

- The cache is keyed by goal string exactly. Minor variations produce new entries.
- Cached tools still require approval before execution.
