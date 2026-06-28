# Identity Provider Setup

## Supported IdP types

| Protocol | Struct | Mock available |
|---|---|---|
| OIDC | `IdpConfig::oidc()` | `MockOidcIdp` |
| SAML 2.0 | `IdpConfig::saml()` | `MockSamlIdp` |

## OIDC IdP setup checklist

1. Register Ancora as an OAuth2/OIDC client in your IdP.
2. Set the redirect URI to your ACS endpoint.
3. Copy `client_id` and `client_secret` into `IdpConfig::oidc(...)`.
4. Export the signing key (`kid`, modulus `n`, exponent `e`) and register it
   via `JwksStore::add_key`.
5. Set `issuer` to the exact `iss` claim value the IdP will produce.
6. Set `audience` to the `aud` claim value (often your `client_id`).

## SAML IdP setup checklist

1. Create a service provider (SP) entry in your SAML IdP with:
   - Entity ID: e.g. `urn:ancora:sp`
   - ACS URL: e.g. `https://ancora.example.com/acs`
2. Download the IdP metadata (issuer, signing certificate).
3. Pass the issuer to `IdpConfig::saml(...)`.
4. Register the entity ID with `MockSamlIdp::trust_entity(...)`.

## JWKS key rotation

Rotate keys before they expire:

```rust
let new_key = JwkKey::new("key-2025", new_modulus, "AQAB", current_tick, current_tick + 86_400);
store.rotate("key-2024", new_key);
```

Keep a 24-hour overlap: the old key stays active long enough for in-flight
tokens to be validated.

## MFA configuration

Enable MFA per tenant:

```rust
IdpConfig::oidc(...)
    .with_mfa(true)
```

For `MfaEnforcer` server-side enforcement (TOTP, hardware key, push):

```rust
use ancora_auth::{MfaChallenge, MfaEnforcer, MfaMethod};

let mut enforcer = MfaEnforcer::new();
enforcer.require_for_tenant("high-sec-tenant");
let challenge = MfaChallenge::new("ch-001", subject, MfaMethod::Totp, expected_code, now_tick, 300);
enforcer.issue_challenge(challenge);
// Later:
let ok = enforcer.verify_challenge("ch-001", user_code, now_tick);
```

## Testing with mock IdPs

Use `MockOidcIdp` and `MockSamlIdp` for unit and integration tests. Both are
entirely in-memory; no network calls are made. Register valid auth codes or
assertions before calling `exchange`/`validate`.
