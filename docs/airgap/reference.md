# ancora-airgap Reference

Air-gap policy enforcement, offline procedures, and media transfer controls for Ancora enterprise crates.

## Modules

### media

`MediaType` enumerates physical and logical transfer media: `UsbDrive`, `CdRom`, `DvdRom`, `NetworkShare`, `Bluetooth`, `PrintedDocument`, `OpticalFibre`, `HardDrive`.

`MediaControl` maintains per-tenant allow/block lists. A blocked media type is never allowed regardless of the allow list.

### transfer

`TransferRequest` models a request to move data across an air-gap boundary. Status transitions: `Pending` -> `Approved` or `Rejected` -> `Completed` or `Cancelled`. Optional checksum field for integrity verification.

`TransferDirection`: `Inbound` or `Outbound`.

### policy

`AirGapPolicy` evaluates a `TransferRequest` and returns a `PolicyVerdict`:
- `Allow` -- transfer may proceed.
- `Deny(reason)` -- transfer blocked.
- `RequireApproval` -- transfer needs human sign-off.

Supported rules: `block_media`, `require_approval_for`, `block_all_outbound`, `require_checksum`.

### boundary

`AirGapZone` defines a named zone with a `ZoneClassification`: `Public`, `Internal`, `Restricted`, or `TopSecret`. `Restricted` and `TopSecret` zones are flagged by `is_restricted()`.

`AirGapBoundary` manages a collection of zones with lookup and classification filtering.

### procedure

`OfflineProcedure` contains an ordered list of `ProcedureStep` entries. Each step progresses through `Pending` -> `Completed`, `Skipped`, or `Failed`. `progress()` returns a 0.0--1.0 completion ratio.

### audit

`AirGapAuditLog` records `AirGapAuditEntry` values covering 11 action types (transfer lifecycle, procedure events, policy evaluations, and media blocks).

### store

`TransferStore` is a keyed collection of `TransferRequest` values. Supports pending filter, status filter, and per-tenant lookup.

### stats

`AirGapStats::for_tenant(transfers, tenant_id)` computes totals by status, per-media distribution, and `rejection_rate()`.

### report

`AirGapReport::generate(boundary, store, audit, tick)` aggregates a snapshot: zone counts, restricted zone count, transfer totals, pending transfers, and audit entry count.

### builder

`TransferBuilder` and `ZoneBuilder` provide fluent construction of transfers and zones.

### presets

- `strict_airgap_policy(tenant_id)` -- blocks Bluetooth and NetworkShare, requires approval for USB, requires checksum, blocks all outbound.
- `standard_airgap_policy(tenant_id)` -- blocks Bluetooth and NetworkShare, requires approval for USB.
- `restricted_zone(tenant_id)` -- a Restricted classification zone.
- `top_secret_zone(tenant_id)` -- a TopSecret classification zone.
- `data_import_procedure(tenant_id)` -- 5-step offline data import procedure.

## Usage

```rust
use ancora_airgap::media::MediaType;
use ancora_airgap::presets::{data_import_procedure, strict_airgap_policy};
use ancora_airgap::transfer::{TransferDirection, TransferRequest};

let policy = strict_airgap_policy("t1");
let req = TransferRequest::new("r1", "t1", "alice", MediaType::UsbDrive, TransferDirection::Inbound, "firmware update", 1)
    .with_checksum("sha256:abc123");
let verdict = policy.evaluate(&req);
println!("{:?}", verdict);

let mut proc = data_import_procedure("t1");
proc.get_step_mut("s1").unwrap().complete(2);
println!("Progress: {:.0}%", proc.progress() * 100.0);
```
