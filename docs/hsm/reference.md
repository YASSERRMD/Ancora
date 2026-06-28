# ancora-hsm Reference

Hardware security module interface and software mock for Ancora enterprise crates.

## Modules

### slot

`HsmSlot` models a physical or virtual HSM slot. A slot may be empty, have a token present, or have a token absent (removed).

```rust
let mut slot = HsmSlot::new(0, "Primary Slot", "SoftHSM2");
slot.insert_token();
assert!(slot.has_token());
```

`SlotManager` manages a collection of slots with lookup by id.

### key

`HsmKey` represents a cryptographic key stored in an HSM slot. Keys are non-extractable and sensitive by default.

`HsmKeyAlgorithm`: `Aes128`, `Aes256`, `Rsa2048`, `Rsa4096`, `EcdsaP256`, `EcdsaP384`, `Ed25519`.

`KeyClass`: `SecretKey`, `PublicKey`, `PrivateKey`.

### session

`HsmSession` tracks the state of an HSM session (`Open`, `LoggedIn`, `LoggedOut`, `Closed`). `SessionManager` opens and closes sessions with auto-incrementing ids.

### mock

`SoftHsm` is a pure-Rust software HSM mock. It generates keys with auto-incrementing handles, performs byte-shuffle encryption/decryption, and signs by appending the key handle to data. Suitable for offline tests and local development.

### audit

`HsmAuditLog` records `HsmAuditEntry` values for each HSM operation. Supports filtering by slot, operation kind, and failure status.

### stats

`HsmStats::from_keys(keys)` computes total key count, per-algorithm distribution, extractable count, sensitive count, and `sensitive_ratio()`.

### policy

`HsmPolicy` enforces constraints: whether extractable keys are allowed, a minimum key bit count, and a blocklist of algorithms. Use `.block_algorithm()` and `.allow_extractable()` as builder methods.

### report

`HsmReport::generate(hsm, slots, sessions, audit, tick)` aggregates snapshot data across all modules into a single struct.

### builder

`HsmKeyBuilder` and `SlotBuilder` provide a fluent API for constructing keys and slots with explicit field overrides.

### presets

Ready-made helpers:
- `aes256_key(hsm, slot_id, tick)` -- generates an AES-256 key.
- `ed25519_signing_key(hsm, slot_id, tick)` -- generates an Ed25519 signing key.
- `default_slot()` -- returns slot 0 with a token inserted.
- `strict_hsm_policy()` -- blocks RSA-2048, requires 256-bit minimum.

## Usage

```rust
use ancora_hsm::mock::SoftHsm;
use ancora_hsm::key::HsmKeyAlgorithm;
use ancora_hsm::presets::{aes256_key, strict_hsm_policy};

let mut hsm = SoftHsm::new();
let handle = aes256_key(&mut hsm, 0, 1);
let policy = strict_hsm_policy();
let key = hsm.get_key(handle).unwrap();
assert!(policy.is_allowed(key));
```
