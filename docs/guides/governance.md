# Governance and Sovereignty Guide

Ancora's `ancora-policy` crate provides declarative governance controls that
can be attached to any agent or tool. Policies are enforced by the engine
before egress calls are made; they are not advisory.

## Policy construction

```rust
use ancora_policy::policy::Policy;

// Deny all outbound egress (air-gapped deployment).
let policy = Policy::new().air_gapped();

// Allow only calls to an EU data-residency endpoint.
let policy = Policy::new()
    .allow_endpoint("https://eu.api.example.com");

// Require PII redaction and audit logging.
let policy = Policy::new()
    .allow_endpoint("https://eu.api.example.com")
    .require_pii_redaction(true)
    .require_audit(true);
```

## Air-gapped mode

`Policy::air_gapped()` blocks all outbound network calls unconditionally.
Any attempt to call `check_endpoint` returns `PolicyError::EgressBlocked`.
The `allowed_endpoints` list is ignored in this mode.

Use air-gapped mode for:

- Regulated workloads that must not contact external APIs.
- Offline / edge deployments with no internet access.
- Security-sensitive environments where egress must be proven-zero.

The air-gapped guarantee is enforced at the policy layer. For defence in depth,
combine it with a network-level firewall or eBPF egress filter.

## Data residency

Many regulatory frameworks (GDPR, PIPL, HIPAA) require that data stay within
a specific jurisdiction. Use `allow_endpoint` with jurisdiction-specific
prefixes:

```rust
// EU residency: only the Frankfurt endpoint is allowed.
let policy = Policy::new()
    .allow_endpoint("https://eu-central-1.api.example.com");
```

Attempts to reach any other endpoint return `PolicyError::ResidencyViolation`.

## PII redaction

When `require_pii_redaction = true`, the engine runs `pii::redact_journal`
before committing events to the journal. Detected PII patterns (email
addresses, card numbers, government identifiers) are replaced with
`[REDACTED]`.

PII detection is pattern-based. For regulated data, supplement with a
dedicated DLP service scanning journal events on egress.

## Audit logging

When `require_audit = true`, every tool call that passes the policy check
raises `PolicyError::AuditRequired` if the caller has not confirmed that the
action will be written to an audit log. Wire this to your audit pipeline:

```rust
match check_audit_required(&policy, "tool_call") {
    Ok(()) => { /* audit not required, proceed */ }
    Err(PolicyError::AuditRequired(action)) => {
        audit_log.write(&action, &input_json);
        // proceed after logging
    }
    Err(e) => return Err(e),
}
```

## Attaching policies to agents

Use `PolicyAttachment` to associate a policy with a named agent or tool:

```rust
use ancora_policy::policy::{Policy, PolicyAttachment};

let attachment = PolicyAttachment {
    target_name: "sensitive-agent".into(),
    policy: Policy::new().air_gapped(),
};
```

The attachment can be stored in a governance registry and looked up by agent
name before each run.

## Sovereignty checklist

Use this checklist before deploying an agent in a regulated environment:

- [ ] `Policy::air_gapped()` is set for workloads that must not contact
  external APIs.
- [ ] All approved external endpoints are listed in `allow_endpoint`.
- [ ] `require_pii_redaction = true` is set for workloads that handle
  personal data.
- [ ] `require_audit = true` is set for workloads subject to audit requirements.
- [ ] A network-level control (firewall, service mesh) backs up the policy
  engine for defence in depth.
- [ ] The journal backend is in the same jurisdiction as the data it records.
- [ ] Bearer tokens for the MCP server are rotated on a schedule.

## Related

- [Threat model](../security/threat-model.md) -- security posture and T3
  (egress) analysis
- [Durability guide](./durability.md) -- journal storage and compliance
- [Observability guide](./observability.md) -- audit-log integration
