# SSO Integration Guide

## Overview

`ancora-auth` provides offline-first OIDC and SAML integration with no network
dependencies. All flows use abstract monotonic u64 ticks instead of wall-clock
time, making them deterministic and testable.

## OIDC Integration

### Prerequisites

- An OIDC-compliant identity provider (e.g. Keycloak, Okta, Azure AD)
- Your `client_id` and `client_secret` from the IdP
- The IdP's JWKS endpoint (key material loaded at startup, not fetched at runtime)

### Configuration

```rust
use ancora_auth::{IdpConfig, IdpRegistry};

let mut registry = IdpRegistry::new();
registry.register(
    IdpConfig::oidc(
        "acme-corp",
        "https://idp.acme.com",
        "my-client-id",
        "my-client-secret",
    )
    .with_mfa(true),
);
```

### Auth code exchange

```rust
use ancora_auth::{MockOidcIdp, OidcAuthCode};

// In production, replace MockOidcIdp with your real IdP adapter
let auth_code = OidcAuthCode::new(code_from_redirect, "acme-corp", mfa_verified);
let token = idp.exchange(&auth_code, &config, &kid, current_tick)?;
```

### MFA enforcement

Set `mfa_required = true` on the `IdpConfig`. The OIDC flow returns
`OidcError::MfaRequired` if `mfa_verified = false` in the `OidcAuthCode`.

## SAML Integration

### Configuration

```rust
use ancora_auth::IdpConfig;

let config = IdpConfig::saml(
    "gov-tenant",
    "https://saml-idp.gov.example.com",
    "urn:ancora:sp",
    "https://ancora.gov.example.com/acs",
)
.with_mfa(true);
```

### Assertion validation

```rust
use ancora_auth::{MockSamlIdp, SamlAssertion};

let idp = MockSamlIdp::new("https://saml-idp.gov.example.com")
    .trust_entity("urn:ancora:sp");
let token = idp.validate(&assertion, &config, current_tick)?;
```

Assertions must be:
- Signed (`with_signed(true)`)
- Within validity period (`valid_from_tick..valid_until_tick`)
- From a trusted entity (added via `trust_entity`)
- MFA-contexted if `config.mfa_required`

## Multi-tenant setup

Each tenant gets its own `IdpConfig` in the `IdpRegistry`. Look up by
`tenant_id` at authentication time.

```rust
let config = registry.get(&tenant_id)?;
```

## Logout and session cleanup

Obtain the session from `SessionStore`, call `store.logout(session_id)`, then
revoke the token via `RevocationStore::revoke`.
