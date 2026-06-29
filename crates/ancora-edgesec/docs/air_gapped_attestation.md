# Air-Gapped Attestation

## Overview

Air-gapped edge devices cannot communicate over a network to a remote attestation server. The `ancora-edgesec` crate supports offline attestation via deterministic proof tokens that can be physically transported (USB drive, printed QR code, manual data entry) to an offline verifier.

## How It Works

1. The device generates an `AirGappedProof` from its device ID, boot hash, logical tick, and a nonce.
2. All computation is deterministic and purely local -- no network, no randomness.
3. The proof is bundled into an `AirGappedAttestationBundle` with boot hash and device metadata.
4. The bundle is serialized to a text format suitable for physical transport.
5. The verifier re-derives the expected proof from the same inputs and compares.

## Generating an Offline Proof

```rust
use ancora_edgesec::airgap_proof::{AirGappedProof, AirGappedAttestationBundle};

let device_id = "airgap-device-001";
let boot_hash = vec![0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE, 0xBA, 0xBE];
let tick = 42u64;
let nonce = 12345u64; // Agreed upon out-of-band with the verifier

let proof = AirGappedProof::generate(device_id, &boot_hash, tick, nonce);
let bundle = AirGappedAttestationBundle::new(proof, boot_hash.clone(), "model=v1 config=prod");

// Serialize for transport
let text = bundle.to_text();
println!("{}", text);
```

## Verifying an Offline Proof

The verifier receives the text bundle (e.g., via USB), reconstructs the inputs, and calls `verify`:

```rust
let verified = AirGappedProof::verify(device_id, &boot_hash, tick, nonce, &proof.proof_token);
assert!(verified);
```

Or use the bundle's built-in verification:

```rust
assert!(bundle.verify_offline());
```

## Security Properties

- **Replay prevention**: The nonce is agreed upon out-of-band (e.g., issued by the verifier before the device disconnected).
- **Determinism**: The same inputs always produce the same proof -- verifier can confirm without needing the device online.
- **Tamper evidence**: Any change to boot hash, device ID, tick, or nonce will produce a different proof that fails verification.
- **No secrets in transport**: The proof token does not leak private key material.

## Nonce Agreement Protocol

1. Before the device goes air-gapped, the verifier issues a nonce and records it alongside the device ID.
2. The device stores the nonce in `EncryptedLocalStorage`.
3. When attestation is required, the device retrieves the nonce and generates the proof.
4. The verifier uses the stored nonce to verify the presented proof.

## Limitations

- The simulated proof uses XOR-based derivation for pure-std compatibility. In production, replace with HMAC-SHA256 or similar.
- Nonce freshness must be managed out-of-band; stale nonces enable replay within the nonce's validity window.
