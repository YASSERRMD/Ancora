# Tenant Guide

The `ancora-tenant` crate provides per-tenant resource quotas, namespace isolation, lifecycle management, and admission control for Ancora's multi-tenant enterprise deployment.

## Core concepts

**Tenant** is the top-level entity representing a customer or isolated workspace. Each tenant has a unique `id`, a human-readable `name`, a `TenantStatus` (Active/Suspended/Deleted), a `created_tick`, and optional metadata key-value pairs.

**ResourceQuota** defines the upper bounds on resources a tenant may consume: agents, tasks, memory, CPU, secrets, and log entries. Use `ResourceQuota::standard()` for typical multi-tenant defaults, `ResourceQuota::restricted()` for free-tier limits, or `ResourceQuota::unlimited()` for internal or admin tenants.

**ResourceUsage** tracks current consumption for a tenant. Update usage fields as resources are allocated and released.

**AdmissionController** evaluates a proposed resource delta against the current quota and usage, returning `AdmissionDecision::Allow` or `AdmissionDecision::Deny(reason)`.

**TenantRegistry** is the central store associating each tenant with its quota, usage, and namespace.

## Quick start

```rust
use ancora_tenant::{TenantBuilder, TenantRegistry, ResourceQuota, AdmissionController};

let mut registry = TenantRegistry::new();
let (tenant, quota) = TenantBuilder::new("acme", "Acme Corp", 1)
    .metadata("plan", "enterprise")
    .quota(ResourceQuota::standard())
    .build();
registry.register(tenant, quota).unwrap();

let usage = registry.usage("acme").unwrap();
let quota = registry.quota("acme").unwrap();
let decision = AdmissionController::check_agents(quota, usage, 1);
```

## Lifecycle

Tenants transition between states:

```
Active -> Suspended -> Active
Active -> Deleted
```

Use `tenant.suspend()`, `tenant.activate()`, `tenant.delete()`. The registry's `require_active` method rejects operations on non-active tenants.

## Runtime quota adjustment

```rust
use ancora_tenant::QuotaUpdate;

// applied to the quota stored in a registry or passed directly
QuotaUpdate::new().agents(20).tasks(200).apply(&mut quota);
```

## Event logging

Record lifecycle events with `TenantEventLog`:

```rust
use ancora_tenant::{TenantEvent, TenantEventKind, TenantEventLog};

let mut log = TenantEventLog::new();
log.record(TenantEvent::new(tick, "acme", TenantEventKind::Registered));
```
