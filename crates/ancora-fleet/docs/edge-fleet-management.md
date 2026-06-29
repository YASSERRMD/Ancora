# Edge Fleet Management

ancora-fleet provides centralized management of many edge devices from a single control plane.

## Concepts

- **Device Identity**: each edge device has a unique `DeviceId`, a human-readable name, and a cryptographic fingerprint. Devices go through a lifecycle: `Pending -> Active -> Revoked/Decommissioned`.
- **Fleet Inventory**: tracks hardware/software properties of every registered device (OS, CPU, memory, disk, installed models, agent version).
- **Config Push**: distribute configuration versions to individual devices or the entire fleet atomically.
- **Model Distribution**: push AI model artifacts with checksum verification to one or all devices.
- **Staged Rollout**: roll out firmware or models incrementally across the fleet using named phases with percentage targets.
- **Health Telemetry**: collect CPU, memory, disk, temperature, uptime, and error counts per device; detect unhealthy devices and compute alert levels.
- **Remote Policy Update**: push enforcement policies (TLS settings, key lengths, firewall rules) to devices.
- **Device Decommission**: safely remove devices from the active fleet, revoking their credentials.
- **Air-gapped Fleet**: bundle all required artifacts into an offline bundle for devices without internet connectivity.
- **Fleet Dashboard**: aggregate all of the above into a single JSON snapshot for dashboards and monitoring.

## Quick Start

```rust
use ancora_fleet::registration::{DeviceId, DeviceRegistry, RegistrationRequest};
use std::collections::HashMap;

let mut registry = DeviceRegistry::new();
let req = RegistrationRequest {
    device_id: DeviceId::new("edge-001"),
    name: "Primary Edge Node".into(),
    fingerprint: "sha256:abc...".into(),
    metadata: HashMap::new(),
};
let resp = registry.register(req);
assert!(resp.success);
```

## Lifecycle

```
RegistrationRequest -> DeviceRegistry::register() -> DeviceIdentity{Active}
                                                         |
                                          policy/config/model push
                                                         |
                                    DecommissionService::decommission()
                                                         |
                                          DeviceIdentity{Decommissioned}
```

## Fleet-wide Operations

All push/distribute/rollout functions accept a slice of `DeviceId` values and return a `Vec` of records — one per device — enabling fan-out to thousands of devices in a single call.

## Testing

All tests are offline and require no network access:

```
cargo test -p ancora-fleet
```
