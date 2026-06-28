# Namespace Isolation

Each tenant in Ancora has a dedicated `Namespace` -- a scoped key-value store that holds tenant-specific configuration, credentials, and runtime data.

## Isolation guarantees

Two namespaces belonging to different tenants are completely isolated at the API level. The `is_isolated_from` method returns `true` when two namespaces have different `tenant_id` values, making it impossible to accidentally share a reference between tenants within the type system.

The `scoped_key` method prefixes any key with `{tenant_id}::`, so even if two namespaces were somehow merged (e.g., in an external store), keys would remain distinct.

## Usage

```rust
use ancora_tenant::{TenantRegistry, ResourceQuota, Tenant};

let mut registry = TenantRegistry::new();
registry.register(Tenant::new("t1", "Acme", 1), ResourceQuota::standard()).unwrap();

let ns = registry.namespace_mut("t1").unwrap();
ns.set("db_url", "postgres://internal/acme_db");
let url = ns.get("db_url");
```

## Cross-tenant safety

`IsolationChecker::require_same_tenant` returns an error when the subject tenant differs from the resource tenant, preventing cross-tenant data access.

```rust
use ancora_tenant::{IsolationChecker, TenantRegistry};

let result = IsolationChecker::require_same_tenant(&registry, "t1", "t2");
assert!(result.is_err()); // denied: different tenants
```

## Scoped keys for external stores

When writing to an external key-value store (Redis, etcd, etc.), use `namespace.scoped_key(key)` to generate a fully-qualified, tenant-prefixed key that prevents namespace collision:

```text
"acme::db_url" vs "beta::db_url"
```
