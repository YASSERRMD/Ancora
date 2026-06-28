# ancora-secboot Reference

Secure boot process integrity verification with measurement chains, attestation records, sealing policies, and integrity reports for the Ancora platform.

## Core Types

### `Measurement`
A single integrity measurement of a component.

| Field | Type | Description |
|---|---|---|
| `id` | `String` | Measurement identifier |
| `kind` | `MeasurementKind` | Component type |
| `name` | `String` | Component name |
| `digest` | `String` | Integrity digest |
| `tick` | `u64` | Monotonic measurement tick |

### `MeasurementKind`
`Firmware` | `Bootloader` | `Kernel` | `InitRamdisk` | `ConfigFile` | `Application`

## Boot Chain

A `BootChain` records an ordered sequence of measurements for a node.

```rust
let mut chain = BootChain::new("tenant", "node-id");
chain.add_step(Measurement::new("m1", MeasurementKind::Kernel, "vmlinuz", "abc123", tick));
let kinds = chain.present_kinds();
let status = chain.status();
```

### `ChainStatus`
`Valid` | `Broken` | `Incomplete`

## Boot Policy

`BootPolicy` defines what measurements are required and which digests are trusted.

```rust
let policy = BootPolicy::new("tenant")
    .require_kind(MeasurementKind::Firmware)
    .require_kind(MeasurementKind::Kernel)
    .allow_digest("vmlinuz", "trusted-digest")
    .allow_digest("uefi.bin", "fw-digest");
```

## Integrity Evaluator

```rust
let decision = IntegrityEvaluator::evaluate(&policy, &chain);
match decision {
    IntegrityDecision::Pass => println!("boot chain verified"),
    IntegrityDecision::Fail(reason) => println!("integrity failure: {}", reason),
}
```

## Sealing Store

Bind secrets to a specific boot state digest.

```rust
store.seal("secret-id", "tenant", "my-secret", "expected-digest", tick);
let result = store.unseal("secret-id", "current-digest");
// returns Unsealed(data), PolicyMismatch, or NotSealed
```

## Attestation

```rust
let mut log = AttestationLog::new();
log.record(AttestationRecord::new("id", "tenant", "node", AttestationStatus::Trusted, "quote", tick));
let trusted = log.trusted();
let node_records = log.for_node("node-id");
```

### `AttestationStatus`
`Trusted` | `Untrusted` | `Unknown`

## Audit Log

```rust
log.record(BootAuditEntry::new(tick, "tenant", "node", BootEvent::ChainValidated, "subject", true, "ok"));
log.failures();
log.for_tenant("tenant");
```

### `BootEvent`
`MeasurementAdded` | `AttestationReceived` | `PolicyChecked` | `SealOperation` | `UnsealOperation` | `ChainValidated`

## Integrity Report

```rust
let report = IntegrityReport::generate(&policy, &chain, &attestations, tick);
assert!(report.is_fully_trusted());
```

## Stats

```rust
let stats = BootStats::from(&chain, &attestations);
println!("Trust rate: {:.2}%", stats.trust_rate() * 100.0);
```

## Builder

```rust
let m = MeasurementBuilder::new("m1", "vmlinuz")
    .kind(MeasurementKind::Kernel)
    .digest("deadbeef")
    .tick(100)
    .build();
```

## Presets

```rust
let strict = strict_boot_policy("tenant");     // requires firmware + bootloader + kernel
let permissive = permissive_boot_policy("tenant"); // allows any digest
let kernel = kernel_only_policy("tenant");    // requires kernel only
```

## Offline-First Design

All types use `u64` monotonic ticks. No network calls. No third-party dependencies.
