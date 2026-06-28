# Secrets Guide

The `ancora-secrets` crate provides vault-style secret storage with versioning, rotation, TTL expiry, access logging, and per-tenant isolation.

## Core concepts

**Secret** is a named, versioned credential or configuration value. Secrets are stored at vault-style paths (`db/password`, `service/api-key`, `infra/tls.cert`). Each secret maintains a history of `SecretVersion` values, with one designated as active.

**SecretStore** is the primary data structure. It is tenant-scoped: each (tenant_id, path) pair is an independent slot. Two tenants may use the same path with no conflict.

**SecretVersion** holds the actual value plus metadata and a status (Active, Rotated, Deleted, Expired).

**RotationPolicy** wraps the rotation workflow: it writes a new version, marks prior active versions as Rotated, and prunes old versions beyond a configurable max.

**SecretAccessLog** records Read/Write/Rotate/Delete events for audit trail and anomaly detection.

**ExpiryChecker** identifies secrets that have exceeded their TTL by comparing `created_tick + ttl_ticks` against the current monotonic tick.

## Quick start

```rust
use ancora_secrets::{SecretKind, SecretStore, RotationPolicy};

let mut store = SecretStore::new();
store.create("acme", "db/password", SecretKind::DatabaseCredential, "initial-pass", 1).unwrap();

// Read active value
let secret = store.read("acme", "db/password").unwrap();
println!("active: {}", secret.active_value().unwrap());

// Rotate
let policy = RotationPolicy::default_policy();
policy.rotate(&mut store, "acme", "db/password", "new-pass", 2).unwrap();
```

## Path rules

Paths must satisfy:
- Non-empty
- Max 256 characters
- Characters: `[a-zA-Z0-9/._-]` only (no spaces, no special chars)
- Must not start or end with `/`

## TTL expiry

```rust
let mut store = SecretStore::new();
store.create("t1", "api/key", SecretKind::ApiKey, "sk-abc", 100).unwrap();
let secret = store.read_mut("t1", "api/key").unwrap();
secret.ttl_ticks = Some(500); // expires at tick 600
```

Use `ExpiryChecker::expired_paths(&store, "t1", current_tick)` to list secrets that need renewal.

## Access logging

```rust
use ancora_secrets::{AccessKind, AccessRecord, SecretAccessLog};

let mut log = SecretAccessLog::new();
log.record(AccessRecord::new(tick, "acme", "db/password", "alice", AccessKind::Read));
```
