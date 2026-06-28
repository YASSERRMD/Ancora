# Upgrade Notes

## From Phases 160 and earlier

### Crate additions

The following crates were added in Phases 161-200.  Add them to `Cargo.toml`
as needed:

```toml
ancora-orchestrate  = { path = "crates/ancora-orchestrate" }
ancora-memcon       = { path = "crates/ancora-memcon" }
ancora-toolsynth    = { path = "crates/ancora-toolsynth" }
ancora-skills       = { path = "crates/ancora-skills" }
ancora-lh           = { path = "crates/ancora-lh" }
ancora-coord        = { path = "crates/ancora-coord" }
ancora-guard        = { path = "crates/ancora-guard" }
ancora-reason       = { path = "crates/ancora-reason" }
ancora-ageval       = { path = "crates/ancora-ageval" }
ancora-preset       = { path = "crates/ancora-preset" }
ancora-advbench     = { path = "crates/ancora-advbench" }
ancora-auth         = { path = "crates/ancora-auth" }
```

### API changes since Phase 160

- `AgentTask` no longer has an `.index` field; use `.task_id` for identity.
- `fan_out` now takes `Vec<serde_json::Value>` inputs (not `Vec<String>`).
- `AllowDenyGuardrail` is constructed with `::deny(tools)` or `::allow_only(tools)`,
  not `::new()`.
- `ConsolidationJob` is a plain struct with public fields, not constructed with
  `::new()`.  Use struct initialization syntax.
- `CitationStore::add` takes `&str, String` (not `String, String`).
- `ReasoningJournal::record` takes `(tick, ReasoningEvent)`, not a plain string.

### Crates added in Phase 201

- `ancora-auth`: OIDC/SAML authentication, JWT validation, JWKS rotation,
  session management, service accounts, MFA enforcement, token revocation.

Key types: `IdpConfig`, `JwksStore`, `JwkKey`, `JwtClaims`, `JwtValidator`,
`MockOidcIdp`, `MockSamlIdp`, `ServiceAccountRegistry`, `SessionStore`,
`MfaEnforcer`, `RevocationStore`, `TokenIntrospector`, `AuthAuditLog`.

All flows use u64 ticks, no wall-clock time.  `MockOidcIdp` and `MockSamlIdp`
are drop-in test doubles that require zero network access.

### Test patterns

All test modules within `ancora-advdet`, `ancora-advpar`, `ancora-preset`,
`ancora-advbench`, and `ancora-auth` use `crate::` imports (not the crate
name) because they are internal `#[cfg(test)]` modules, not integration tests.
