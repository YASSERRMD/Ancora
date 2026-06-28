# Service Account Guide

## Overview

Service accounts allow machine-to-machine authentication without a user session.
They authenticate with a pre-shared key hash and receive a scoped token.

## Creating a service account

```rust
use ancora_auth::{ServiceAccount, ServiceAccountRegistry};

let mut registry = ServiceAccountRegistry::new();
registry.register(
    ServiceAccount::new(
        "ci-pipeline",
        "acme-corp",
        "sha256:deadbeef",
        "CI/CD pipeline",
    )
    .with_scope("read:agents")
    .with_scope("write:tasks"),
);
```

## Authenticating

```rust
let token = registry.authenticate(
    "ci-pipeline",
    "sha256:deadbeef",
    3600, // TTL in ticks
    current_tick,
)?;
```

The returned `Token` has `kind = TokenKind::ServiceAccount` and carries the
registered scopes.

## Key management

Store key hashes only (never plaintext keys). Use `SHA-256` or `BLAKE3` of
the raw key. Rotate by creating a new service account entry with the new hash
and disabling the old one.

## Disabling accounts

```rust
registry.disable("ci-pipeline");
```

Subsequent authentication attempts return `ServiceAccountError::Disabled`.

## Token lifetime

Pass a `ttl_ticks` appropriate for the call frequency. Keep TTLs short (< 1 hour
in ticks) and rotate credentials regularly.

## Scope enforcement

Check scopes at the API boundary before authorizing any action:

```rust
if !token.has_scope("write:tasks") {
    return Err(Unauthorized);
}
```

Scopes are simple string identifiers; use `resource:action` convention to keep
them readable.
