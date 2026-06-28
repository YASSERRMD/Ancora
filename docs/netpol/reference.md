# ancora-netpol: Network Egress Policy

`ancora-netpol` provides offline-first network egress policy enforcement for multi-tenant agents.

## Core concepts

### NetworkRule

A rule matches outbound connection requests. Fields:

| Field | Type | Description |
|-------|------|-------------|
| `id` | `String` | Unique rule identifier within a policy |
| `host_pattern` | `String` | Exact host, `*` (any), or `*.domain.com` (wildcard subdomain) |
| `port` | `Option<u16>` | Specific port, or `None` to match any port |
| `protocol` | `Protocol` | `Tcp`, `Udp`, or `Any` |
| `effect` | `Effect` | `Allow` or `Deny` |
| `priority` | `u32` | Lower number = higher priority; first-match-wins within a policy |

Use `RuleBuilder` for ergonomic construction:

```rust
let rule = RuleBuilder::new("allow-api")
    .host("api.internal.corp")
    .port(443)
    .tcp()
    .allow()
    .priority(100)
    .build();
```

### NetworkPolicy

Holds an ordered rule list and a default posture applied when no rule matches.

```rust
let mut policy = NetworkPolicy::deny_by_default("tenant-abc");
policy.add_rule(rule);
```

`add_rule` keeps rules sorted by priority ascending. `bulk_add_rules` loads multiple rules and re-sorts once.

### PolicyEvaluator

Evaluates a `ConnectionRequest` against a `NetworkPolicy`:

```rust
let req = ConnectionRequest::tcp("tenant-abc", "agent-01", "api.internal.corp", 443);
let decision = PolicyEvaluator::evaluate(&policy, &req);
```

Returns `PolicyDecision::Allow`, `PolicyDecision::Deny(reason)`, or `PolicyDecision::NoPolicy`.

## Evaluation semantics

Rules are evaluated in ascending priority order. The **first rule whose host, port, and protocol conditions all match** wins, regardless of its effect. If no rule matches the default posture applies:

- `DenyAll`: returns `Deny("denied by default posture")`
- `AllowAll`: returns `Allow`

## Presets

| Function | Posture | Rules added |
|----------|---------|-------------|
| `allow_https_only(tenant)` | DenyAll | Allow * port 443 TCP priority=100 |
| `allow_internal_only(tenant, domain)` | DenyAll | Allow `*.{domain}` Any priority=100 |
| `block_known_bad(policy, host)` | unchanged | Deny `{host}` Any priority=10 |

## Audit log

`NetpolAuditLog` records every evaluation as an `EvaluationRecord`. Use `denied_for(tenant_id)` and `allowed_for(tenant_id)` to filter. `NetpolStats::from_log(log, tenant_id)` computes aggregated allow/deny rates.

## Validation

`PolicyValidator::validate(policy)` detects:

- `NoRules`: policy has no rules
- `DuplicateId`: two rules share the same id
- `ShadowedRule`: a later rule may never fire because a wildcard deny precedes it
