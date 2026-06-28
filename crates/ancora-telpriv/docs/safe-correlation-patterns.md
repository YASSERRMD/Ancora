# Safe Correlation Patterns

## The Problem

Telemetry systems need to correlate records across spans, logs, and eval runs
(e.g. "show me all telemetry for session X"). But session IDs and user IDs are
PII and cannot appear raw in exported telemetry.

## The Solution - Hashed Correlation Tokens

Use `CorrelationToken` to derive a stable, opaque identifier from the raw value:

```rust
use ancora_telpriv::hashing::CorrelationToken;

let salt = std::env::var("TELEMETRY_SALT").unwrap_or_default();
let token = CorrelationToken::from_raw(&session_id, &salt);

// token.as_str() is safe to export - it cannot be reversed to session_id
span.set_attribute("session.correlation_id", token.as_str());
```

The same raw value + salt always produces the same token, so records from the
same session are still linkable in your observability backend.

## Choosing a Salt

- Store the salt as a deployment secret (e.g. in Kubernetes Secrets or an HSM).
- Rotate the salt periodically to limit the correlation window.
- Use a different salt per environment (dev vs. prod).
- Never log or export the salt.

## What This Provides

- Two records with the same `session.correlation_id` came from the same session.
- Without the salt, the original session ID cannot be recovered from the token.
- Rotating the salt breaks the correlation link for past events.

## What This Does Not Provide

- This is NOT cryptographically secure (FNV-1a is not a secure hash).
- Do not use correlation tokens as access control tokens.
- If the raw value space is small (e.g. a 4-digit PIN), brute force is trivial.
  Use only for high-entropy identifiers like UUIDs.

## Combining with the Allowlist

Add the correlation token attribute to the allowlist so it passes the export
filter:

```rust
al.add_exact("session.correlation_id");
al.add_exact("user.correlation_id");
```
