# ancora-zerotrust Reference

Zero trust per-request identity verification, device trust scoring, continuous authorization, and access policy evaluation.

## Modules

### `identity`
Core identity type.

- `IdentityKind`: `Human | Service | Device | Workload`
- `IdentityStatus`: `Active | Suspended | Revoked`
- `Identity::new(id, tenant_id, kind, tick)`, `add_group`, `suspend`, `revoke`, `is_active`, `in_group`

### `device`
Device posture and trust scoring.

- `TrustLevel`: `Untrusted | Partial | Trusted | FullyTrusted` (orderable)
- `DevicePosture::new(device_id, tenant_id, tick)` with `os_up_to_date`, `antivirus_active`, `disk_encrypted`
- `compute_trust()` -- derives trust from 3 binary checks; `is_trusted()` (>= Trusted)
- `DeviceStore`: `upsert`, `get`, `get_mut`, `trusted`, `for_tenant`, `count`

### `request`
Per-request access context.

- `AccessRequest::new(id, tenant_id, identity_id, resource, action, tick)`
- `.with_device(device_id)`, `.with_context(k, v)`

### `policy`
Zero trust authorization policy.

- `AuthzDecision`: `Allow | Deny(String) | RequireMfa`
- `ZeroTrustPolicy::new(tenant_id).require_device().min_trust(level).mfa_for_group(g).deny_resource(r)`
- `resource_denied(resource)`, `needs_mfa_for(groups)`

### `evaluator`
Per-request authorization evaluation.

- `ZeroTrustEvaluator::evaluate(policy, request, identity, devices)` -> `AuthzDecision`
- Checks: identity active, resource not denied, device trust (if required), MFA (if group matches)

### `session`
Zero trust session management.

- `SessionState`: `Active | Expired | Revoked`
- `ZeroTrustSession::new(id, tenant_id, identity_id, created_tick, expires_tick)`
- `is_valid(tick)`, `refresh_verification(tick)`, `expire`, `revoke`
- `SessionStore`: `insert`, `get`, `get_mut`, `active(tick)`, `for_identity`, `count`

### `audit`
Immutable access audit log.

- `ZtAction`: `AccessGranted | AccessDenied | MfaRequired | SessionCreated | SessionRevoked | DevicePostureChecked | PolicyEvaluated`
- `ZtAuditLog`: `record`, `count`, `for_tenant`, `for_identity`, `denied`, `all`

### `stats`
Identity aggregate statistics.

- `ZeroTrustStats::for_tenant(identities, tenant_id)` -> `total_identities, active_identities, suspended_identities, by_kind`

### `builder`
Fluent builders.

- `IdentityBuilder::new(id, tenant_id, kind).tick().group().build()`
- `SessionBuilder::new(id, tenant_id, identity_id).created_at().expires_at().device().build()`
- `make_request(id, tenant_id, identity_id, resource, action, tick)`

### `presets`
Pre-configured policies.

- `strict_policy(tenant_id)` -- requires FullyTrusted device, MFA for admin+finance
- `standard_policy(tenant_id)` -- requires Trusted device, MFA for admin
- `permissive_policy(tenant_id)` -- no device or MFA requirements

### `report`
Cross-data tenant report.

- `ZeroTrustReport::generate(identities, sessions, devices, audit, tenant_id, tick)`

### `summary`
Tenant health summary.

- `ZeroTrustSummary::generate(identities, sessions, audit, tenant_id, tick)` -> `is_healthy` (zero denied)

## Quick start

```rust
use ancora_zerotrust::builder::IdentityBuilder;
use ancora_zerotrust::identity::IdentityKind;
use ancora_zerotrust::presets::standard_policy;
use ancora_zerotrust::device::DeviceStore;
use ancora_zerotrust::evaluator::ZeroTrustEvaluator;
use ancora_zerotrust::request::AccessRequest;

let identity = IdentityBuilder::new("alice", "acme", IdentityKind::Human).build();
let policy = standard_policy("acme");
let request = AccessRequest::new("r1", "acme", "alice", "api/data", "GET", 1000);
let devices = DeviceStore::new();
let decision = ZeroTrustEvaluator::evaluate(&policy, &request, &identity, &devices);
println!("{:?}", decision);
```
