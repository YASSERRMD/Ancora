# ancora-keymgmt Reference

Key lifecycle management with HSM integration stub, rotation policies, and audit trails for the Ancora platform.

## Core Types

### `CryptoKey`
Represents a single versioned cryptographic key.

| Field | Type | Description |
|---|---|---|
| `id` | `String` | Key identifier |
| `tenant_id` | `String` | Owning tenant |
| `algorithm` | `KeyAlgorithm` | Crypto algorithm |
| `purpose` | `KeyPurpose` | Key usage intent |
| `status` | `KeyStatus` | Lifecycle status |
| `version` | `u32` | Version counter, starts at 1 |
| `created_tick` | `u64` | Monotonic creation tick |
| `expires_tick` | `Option<u64>` | Optional expiry tick |
| `key_material` | `String` | Key bytes (cleared on destroy) |

### `KeyAlgorithm`
`Aes256` | `Rsa2048` | `Rsa4096` | `EcdsaP256` | `EcdsaP384` | `Ed25519` | `Hmac256`

### `KeyPurpose`
`Encryption` | `Signing` | `Authentication` | `KeyWrapping`

### `KeyStatus`
`Active` | `Inactive` | `Compromised` | `Destroyed` | `PendingDeletion`

## Key Store

`KeyStore` provides multi-version, tenant-isolated key storage.

```rust
let mut store = KeyStore::new();
store.create(key)?;
let active = store.get_active("tenant", "key-id")?;
let v1 = store.get_version("tenant", "key-id", 1)?;
store.list_tenant_active("tenant");
store.total_key_ids();
```

## Key Rotation

```rust
let new_version = rotate_key(&mut store, "tenant", "key-id", "new-material", tick)?;
// Previous version becomes Inactive; new version is Active with version+1
```

### `RotationPolicy`
```rust
let policy = RotationPolicy::new(5).with_rotation_interval(1000);
if policy.should_rotate(&key, current_tick) {
    rotate_key(&mut store, ...);
}
```

## HSM Stub

Simulates hardware-backed key generation for offline testing.

```rust
let mut hsm = HsmStub::new(HsmConfig::software());
let key = hsm.generate_key("id", "tenant", KeyAlgorithm::Aes256, KeyPurpose::Encryption, tick);
assert!(HsmConfig::cloud_kms(1).is_hardware_backed());
```

### `HsmBackend`
`Software` | `CloudKms` | `Pkcs11` | `Tpm`

## Audit Log

```rust
let mut log = KeyAuditLog::new();
log.record(KeyAuditEntry::new(tick, "tenant", "key-id", version, KeyOperation::Create, "alice", true));
log.for_key("key-id");
log.for_tenant("tenant");
log.rotations_for("key-id");
```

### `KeyOperation`
`Create` | `Read` | `Rotate` | `Deactivate` | `Destroy` | `Compromise`

## Expiry Checker

```rust
let expired = ExpiryChecker::expired_keys(&store, "tenant", current_tick);
let soon = ExpiryChecker::expiring_soon(&store, "tenant", current_tick, warning_ticks);
```

## Stats

```rust
let stats = KeyStats::for_tenant(&store, "tenant");
println!("{} active keys", stats.total_active);
println!("{:?}", stats.by_algorithm);
```

## Builder

```rust
let key = KeyBuilder::new("my-key", "tenant-1")
    .algorithm(KeyAlgorithm::EcdsaP256)
    .purpose(KeyPurpose::Signing)
    .tick(100)
    .expires_at(10000)
    .material("secret")
    .build();
```

## Presets

```rust
let enc_key = aes256_encryption_key("k1", "tenant", tick);
let sig_key = ed25519_signing_key("k2", "tenant", tick);
let auth_key = rsa2048_auth_key("k3", "tenant", tick);
let hmac_key = hmac256_signing_key("k4", "tenant", tick);
let ephem = ephemeral_key("k5", "tenant", tick, 500);
```

## Validator

```rust
let issues = KeyValidator::validate_key(&key, current_tick);
let tenant_issues = KeyValidator::validate_tenant(&store, "tenant", current_tick);
assert!(KeyValidator::is_valid_key(&key, current_tick));
```

## Offline-First Design

All types use `u64` monotonic ticks. No network calls. No third-party dependencies.
