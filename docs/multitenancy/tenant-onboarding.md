# Tenant Onboarding Guide

## Create a tenant

```rust
use ancora_multitenancy::{TenantRegistry, TenantConfig};

let mut registry = TenantRegistry::new();
let tenant_id = registry.create("acme", TenantConfig {
    provider_allowlist: vec!["openai".to_string()],
    residency_region: Some("eu-west".to_string()),
    max_workers: 10,
});
```

## Attach tenant context to an operation

```rust
use ancora_multitenancy::TenantContext;

let ctx = TenantContext::new(tenant_id.clone());
let journal_key = ancora_multitenancy::journal_scope::journal_key(&ctx, "run-abc");
```

## Enforce access before queuing a run

```rust
use ancora_multitenancy::TenantIsolation;

let iso = TenantIsolation::new(&registry);
iso.assert_active(&tenant_id)?;
iso.assert_provider_allowed(&tenant_id, "openai")?;
iso.assert_residency(&tenant_id, "eu-west")?;
```

## Suspend a tenant

```rust
registry.suspend(&tenant_id)?;
// All new runs from this tenant will now return TenantError::Suspended.
```

## Delete a tenant

```rust
registry.delete(&tenant_id)?;
// Soft-delete. The record remains for audit purposes.
```

## Kubernetes

In Kubernetes mode, each tenant maps to an `AncoraTenant` custom resource.
The operator provisions an isolated namespace and role binding automatically.
See [crd-reference-and-install.md](../operator/crd-reference-and-install.md).
