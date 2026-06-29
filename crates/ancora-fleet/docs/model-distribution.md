# Model Distribution

ancora-fleet supports distributing AI model artifacts to edge devices reliably and verifiably.

## Artifact Structure

A `ModelArtifact` contains:

| Field | Description |
|---|---|
| `model_id` | Unique model name (e.g. `llm-tiny-v2`) |
| `version` | Semantic version string |
| `size_bytes` | Expected artifact size |
| `checksum` | Hex-encoded SHA-256 digest |

## Distribution Flow

1. Create a `ModelArtifact` with a non-empty checksum.
2. Call `ModelDistributionService::distribute(device_id, artifact)` for a single device, or `distribute_to_fleet(device_ids, artifact)` for many devices.
3. The service simulates transfer then sets status to `Verified` if the checksum is present, or `Failed` if it is missing.
4. Query `is_verified(device_id, model_id)` to confirm delivery.
5. Use `verified_devices(model_id)` to list all devices that have confirmed the model.

## Example

```rust
use ancora_fleet::model_dist::{ModelArtifact, ModelDistributionService};
use ancora_fleet::registration::DeviceId;

let mut svc = ModelDistributionService::new();
let artifact = ModelArtifact::new("llm-v3", "3.0.0", 500_000_000, "deadbeef1234");

let device_ids: Vec<DeviceId> = vec![DeviceId::new("edge-001"), DeviceId::new("edge-002")];
let records = svc.distribute_to_fleet(&device_ids, &artifact);
for r in &records {
    println!("{:?}: {:?}", r.device_id, r.status);
}
```

## Checksum Verification

The fleet manager verifies each artifact's checksum after transfer. If the checksum field is empty the distribution is marked `Failed`. In production, use a strong hash (SHA-256 or BLAKE3) computed over the raw model bytes before creating the `ModelArtifact`.

## Staged Model Rollout

Combine model distribution with the staged rollout engine to push a new model to a canary group first, validate health telemetry, then proceed to the full fleet.
