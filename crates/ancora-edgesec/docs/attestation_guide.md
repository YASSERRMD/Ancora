# Attestation Guide

## What is Attestation?

Attestation is the process of cryptographically proving that an edge device is running expected, unmodified software and configuration. The `ancora-edgesec` crate provides three layers of attestation:

1. **Secure boot attestation** - proves the boot chain is unmodified
2. **Model integrity attestation** - proves the AI model has not been tampered with
3. **Config integrity attestation** - proves the runtime configuration is authoritative

## Secure Boot Attestation

Register boot measurements and attest them at startup:

```rust
use ancora_edgesec::boot::{BootMeasurement, SecureBootAttestation};

let measurements = vec![
    BootMeasurement::new("bootloader", expected_hash.clone(), measured_hash.clone()),
    BootMeasurement::new("kernel", expected_k.clone(), measured_k.clone()),
];
let attestation = SecureBootAttestation::attest("my-device", measurements, tick);
assert!(attestation.is_verified());
```

Use `SecureBootHook` to plug into a pre-boot validation pipeline:

```rust
use ancora_edgesec::boot::SecureBootHook;

let hook = SecureBootHook::new("policy-check", |measurements| {
    measurements.iter().all(|m| m.is_valid())
});
assert!(hook.run(&measurements));
```

## Model Integrity Attestation

```rust
use ancora_edgesec::attestation::{AttestationRegistry, attest_model};

let mut registry = AttestationRegistry::new();
attest_model(&mut registry, "llm-v2", expected_digest, measured_digest, tick);
let record = registry.get("llm-v2").unwrap();
assert!(record.is_valid());
```

## Config Integrity Attestation

```rust
use ancora_edgesec::attestation::{AttestationRegistry, attest_config};

let mut registry = AttestationRegistry::new();
attest_config(&mut registry, "prod-config", expected, measured, tick);
assert!(registry.all_valid());
```

## Remote Attestation Report

Combine all attestation data into a remote attestation report:

```rust
use ancora_edgesec::report::AttestationReport;

let report = AttestationReport::generate(
    "device-id",
    tick,
    boot_status,
    model_valid,
    config_valid,
    tamper_events,
    nonce,
);
println!("{}", report.to_text());
```

## Air-Gapped Attestation

For offline environments, use `AirGappedProof` to generate and verify proofs without network access. See [air-gapped attestation](air_gapped_attestation.md).
