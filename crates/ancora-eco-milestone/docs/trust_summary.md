# Trust and Governance Summary

## Plugin Sandboxing
Level: High

WASM sandbox with capability-based permissions; no ambient authority granted.

## Supply Chain
Level: High

All artifacts signed with ECDSA P-256. Provenance tracked via SLSA level 2.

## Audit Logging
Level: High

Tamper-evident append-only audit log with HMAC chain.

## Access Control
Level: High

RBAC + ABAC enforced at every API boundary.

## Secret Management
Level: High

Secrets stored in HSM-backed vault. Never logged or serialized.

---

Governance score: 100%
