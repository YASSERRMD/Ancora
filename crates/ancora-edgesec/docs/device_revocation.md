# Device Revocation

## Overview

Device revocation is the process of invalidating a compromised, decommissioned, or policy-violating edge device. The `DeviceRevocationList` (DRL) provides a local, network-independent revocation mechanism.

## Revocation Reasons

| Reason | Description |
|--------|-------------|
| `KeyCompromised` | The device's private key was leaked or stolen |
| `TamperDetected` | Physical or software tampering was detected |
| `PolicyViolation` | Device violated a security policy |
| `Decommissioned` | Device was deliberately retired |
| `UnknownReason` | Catch-all for unusual circumstances |

## Revoking a Device

```rust
use ancora_edgesec::revocation::{DeviceRevocationList, RevocationReason};

let mut drl = DeviceRevocationList::new();
drl.revoke(
    "edge-device-001",
    RevocationReason::KeyCompromised,
    "private key found in log file",
    tick,
);
assert!(drl.is_revoked("edge-device-001"));
```

## Checking Revocation Before Operations

All security operations should check the DRL before proceeding:

```rust
if drl.is_revoked(&device_id) {
    // Reject the operation
    return Err("device is revoked");
}
```

## Inspection

```rust
// Get a revocation record
if let Some(record) = drl.get_record("edge-device-001") {
    println!("Revoked at tick {} for: {}", record.tick, record.reason);
}

// Count revoked devices
println!("Total revoked: {}", drl.revoked_count());
```

## Re-enrollment

A revoked device can be re-enrolled after remediation using `unrevoke`. This should only be done after the root cause is resolved and new keys are provisioned:

```rust
drl.unrevoke("edge-device-001");
assert!(!drl.is_revoked("edge-device-001"));
```

## Integration with Identity Registry

The `DeviceIdentityRegistry` also tracks revocation at the key level. Both should be checked in tandem:

```rust
if identity_registry.is_revoked(&device_id) || drl.is_revoked(device_id.0.as_str()) {
    // Block all operations
}
```
