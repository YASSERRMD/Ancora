# Edge Security Model

## Overview

The `ancora-edgesec` crate implements a layered security model for edge devices operating within the Ancora agent framework. Edge devices operate in environments with limited connectivity, constrained resources, and elevated physical-access risk.

## Core Principles

1. **Zero-trust by default**: No device is trusted without cryptographic proof.
2. **Minimal egress**: All outbound connections are blocked unless explicitly permitted.
3. **Continuous attestation**: Boot, model, and config integrity are verified at every lifecycle stage.
4. **Tamper detection**: Any deviation from expected state triggers an alert and is recorded.
5. **Revocability**: Any device can be instantly revoked without network round-trips.

## Security Layers

### Layer 1: Device Identity
Each edge device holds a unique key pair (`DeviceKeyPair`) generated from a device identifier. The `DeviceIdentityRegistry` maintains the authoritative mapping of device IDs to public keys and revocation state.

### Layer 2: Secure Boot Attestation
The `SecureBootAttestation` records boot measurements for each boot stage component. A `SecureBootHook` can be registered to run pre-boot integrity checks before any agent code executes.

### Layer 3: Artifact Attestation
Model integrity (`attest_model`) and configuration integrity (`attest_config`) are verified by comparing expected digests to measured values. An `AttestationRegistry` tracks results per artifact.

### Layer 4: Encrypted Local Storage
All local state is stored via `EncryptedLocalStorage` using a device-specific `StorageKey`. Data at rest is encrypted; plaintext never touches the filesystem directly.

### Layer 5: Tamper Detection
The `TamperMonitor` records tamper events (hash mismatches, unexpected reboots, clock skew, etc.) and exposes per-device tamper status for reporting.

### Layer 6: Remote Attestation Report
The `AttestationReport` aggregates boot status, artifact validity, and tamper events into a signed (simulated) report that can be transmitted to a remote verifier.

### Layer 7: Device Revocation
The `DeviceRevocationList` (DRL) maintains a list of compromised or decommissioned devices. Revocation is checked before any operation is permitted.

### Layer 8: Egress Control
`EdgeEgress` enforces a zero-egress-by-default policy. Outbound connections require explicit allowlist entries.

## Threat Model

| Threat | Mitigation |
|--------|-----------|
| Physical tampering | Tamper detection + secure boot attestation |
| Key leakage | Device revocation list |
| Model poisoning | Model integrity attestation |
| Config injection | Config integrity attestation |
| Data exfiltration | Encrypted local storage + egress control |
| Replay attacks | Nonce-based attestation proofs |
