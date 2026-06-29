# Air-gapped Fleet Operations

ancora-fleet supports edge devices that have no internet connectivity by using offline bundles.

## Overview

An `OfflineBundle` packages all required artifacts (firmware, models, configs, policies) into a single portable unit. Operators transfer the bundle to isolated environments via removable media, secure file transfer, or other out-of-band channels. The fleet manager then applies the bundle to devices in the air-gapped network.

## Bundle Lifecycle

```
Build host (internet) -> create OfflineBundle -> add files -> verify()
         |
    export/ship (USB, air-gap transfer)
         |
Air-gapped controller -> AirGapFleetManager::add_bundle()
                              -> apply_to_fleet(device_ids, bundle_id)
                                     -> verify + apply per device
```

## Creating a Bundle

```rust
use ancora_fleet::airgap::OfflineBundle;

let mut bundle = OfflineBundle::new("release-2.0", "firmware 2.0 + llm-tiny");
bundle.add_file("firmware.bin", std::fs::read("firmware.bin").unwrap());
bundle.add_file("llm-tiny.bin", std::fs::read("llm-tiny.bin").unwrap());

// Verify integrity before shipping
assert!(bundle.verify());
```

Each file's checksum is computed and stored in the bundle manifest at the time of `add_file`. The same checksum is re-verified at application time.

## Applying a Bundle

```rust
use ancora_fleet::airgap::{AirGapFleetManager, OfflineBundle};
use ancora_fleet::registration::DeviceId;

let mut manager = AirGapFleetManager::new();
manager.add_bundle(bundle);

let device_ids = vec![DeviceId::new("air-node-1"), DeviceId::new("air-node-2")];
let records = manager.apply_to_fleet(&device_ids, "release-2.0");
for r in &records {
    println!("{:?}: {:?}", r.device_id, r.status);
}
```

## Integrity Guarantee

If any file's checksum does not match at apply time, the record status is `VerificationFailed` and no changes are applied on that device. This prevents partial or corrupted updates.

## Operational Notes

- Generate and sign bundles on an internet-connected build host.
- Transfer using approved removable media that has been scanned for malware.
- Log every apply operation using the returned `BundleApplyRecord` for audit purposes.
- Rotate bundle IDs with each release to prevent replay attacks.
- Combine with `RemotePolicyService` bundles to enforce updated security policies on air-gapped devices.
