# Approval Workflow

Every synthesized tool requires explicit approval before it can execute write
effects. The approval gate is the final checkpoint between synthesis and use.

## Flow

1. Synthesize: `spec_from_goal(goal)` produces a `ToolSpec`
2. Validate schema: `SchemaValidator::validate(&spec.input_schema)`
3. Check sandbox: `SandboxRunner::execute` enforces effect limits
4. Request approval: `ApprovalGate::approve(tool_name)` (done by human operator)
5. Check approval: `gate.check(tool_name)` before execution
6. Audit: record `AuditEvent::Approved` and `AuditEvent::Executed`

## Revoking Approval

Call `ApprovalGate::revoke(tool_name)` to immediately block a tool from
further execution. Revocation is recorded in the audit trail.
