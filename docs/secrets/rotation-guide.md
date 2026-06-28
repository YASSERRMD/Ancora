# Secret Rotation Guide

## Why rotate

Secrets have a useful life. Rotating credentials limits the blast radius of a compromise: a stolen secret that is already rotated is worthless. Rotation also satisfies compliance requirements (SOC 2, PCI-DSS, ISO 27001) that mandate regular credential refresh.

## How rotation works in ancora-secrets

1. A new `SecretVersion` is created with an incremented version number and status `Active`.
2. All previously `Active` versions are marked `Rotated`.
3. If the total number of versions exceeds `RotationPolicy.max_versions`, the oldest versions are pruned.
4. The `Secret.active_version` pointer is updated to the new version.

This means `secret.active_value()` always returns the most recently rotated value, and historical values remain accessible by iterating `secret.versions` (subject to pruning).

## Usage

```rust
use ancora_secrets::{RotationPolicy, SecretKind, SecretStore};

let mut store = SecretStore::new();
store.create("t1", "db/pass", SecretKind::DatabaseCredential, "v1", 1).unwrap();

// Default policy retains up to 10 versions
let policy = RotationPolicy::default_policy();
policy.rotate(&mut store, "t1", "db/pass", "v2", 2).unwrap();
policy.rotate(&mut store, "t1", "db/pass", "v3", 3).unwrap();

// Custom policy with strict retention
let strict = RotationPolicy::new(2);
strict.rotate(&mut store, "t1", "db/pass", "v4", 4).unwrap();
// Only 2 most recent versions are retained
```

## Automated rotation schedule

Since the log uses monotonic ticks rather than wall-clock time, wire rotation calls to a tick-based scheduler in the agent runtime. A common pattern is to rotate every N ticks as part of the agent's scheduled maintenance cycle.
