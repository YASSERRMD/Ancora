# Multi-Tenancy Model and Isolation Guarantees

## Design

Ancora treats a tenant as a first-class isolation boundary. Every run,
journal entry, memory key, vector collection, and cost record is tagged with
a `TenantId`. The enforcement layer (`TenantIsolation`) is called at every
access point to prevent cross-tenant reads or writes.

## Isolation surfaces

| Surface | Mechanism |
|---------|-----------|
| Journal | Keys prefixed `tenant:<id>:journal:*` |
| Memory | Keys prefixed `tenant:<id>:memory:*` |
| Vector store | Collections prefixed `tenant:<id>:vec:*` |
| Cost ledger | In-memory / store keyed by `TenantId` |
| Runs | `TenantId` field on every `Run`; control plane enforces ownership |

## Policy enforcement

`TenantIsolation` provides three guards:

- `assert_active(id)` - rejects suspended and deleted tenants before queuing a run.
- `assert_owns(requester, owner)` - blocks cross-tenant resource access.
- `assert_provider_allowed(id, provider)` - enforces the per-tenant provider allowlist.
- `assert_residency(id, region)` - enforces the data residency region tag.

An empty provider allowlist means all providers are permitted (opt-in
restriction model).

## Lifecycle

A tenant moves through three states: `Active`, `Suspended`, `Deleted`.
- `Suspended`: existing runs may drain; new runs are rejected.
- `Deleted`: a soft-delete marker; the registry retains the record for audit.

## Guarantees

- No SQL or storage query is issued without a tenant-scoped key prefix.
- Cross-tenant comparison via vector similarity is structurally impossible:
  each tenant has a separate collection name.
- Cost is attributed at the point of LLM call, not post-hoc; no shared pool.
