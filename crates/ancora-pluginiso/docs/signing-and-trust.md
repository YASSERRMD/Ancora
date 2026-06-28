# Plugin Signing and Trust

## Overview

ancora-pluginiso enforces cryptographic signatures on plugin binaries before
loading them. This prevents loading tampered, counterfeit, or supply-chain-
compromised plugins into the host.

## Signature Policy

The `SignaturePolicy` enum controls how strictly signatures are enforced:

| Policy     | Unsigned plugin | Invalid signature |
|------------|-----------------|-------------------|
| `Required` | Rejected        | Rejected          |
| `Optional` | Allowed         | Rejected          |
| `Disabled` | Allowed         | Not checked       |

Production deployments must use `Required`. `Optional` is suitable for
development environments where plugins are built and tested locally.
`Disabled` should only be used in automated testing pipelines that control
the plugin binary directly.

## Trust Store

The `SignatureVerifier` holds a set of `TrustedKey` entries. Each entry has:

- `key_id` - a short string that labels the key (e.g., "releases-2025-q1")
- `public_key_bytes` - the raw public key material

A plugin's detached `PluginSignature` references a `key_id`. Verification
fails immediately with `SignatureError::UnknownKey` if the key is not in the
trust store, preventing key-confusion attacks.

## Key Rotation

To rotate signing keys:

1. Generate a new key pair out of band.
2. Add the new public key to the trust store with a distinct `key_id`.
3. Re-sign all plugins with the new private key.
4. Once all deployed plugins carry the new signature, remove the old key from
   the trust store.

Both old and new keys can coexist in the trust store during a rolling
deployment, so plugins signed with either key continue to load.

## Audit Trail

Every signature verification attempt - success or failure - is appended to
the audit log with:

- `EventKind::SignatureVerified` on success
- `EventKind::SignatureFailed` with the error detail on failure

This provides a complete record of which keys were used to load which plugins
and when any anomalous verification attempts occurred.

## Implementation Note

The `SignatureVerifier::stub_sign` helper is provided for unit testing only.
It is intentionally trivial and MUST NOT be used in production. In a
production deployment, replace it with a constant-time signature primitive
from a well-audited cryptography library (e.g., ed25519-dalek, ring, or the
rustls crypto provider).

## Threat Model

Signing mitigates:

- Supply-chain compromise: a tampered plugin binary will not match the
  original signature.
- Plugin impersonation: a plugin claiming to be "billing-v2" but signed with
  an untrusted key is rejected.
- Downgrade attacks: older plugin versions with known vulnerabilities cannot
  be loaded if their signing keys have been rotated out of the trust store.

Signing does not mitigate:

- A compromised signing key (key management is out of scope for this crate).
- Logic bugs in a legitimately signed plugin (use sandbox policies for that).
