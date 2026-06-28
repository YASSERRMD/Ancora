# Secret Management and Rotation Guide

## Principles

1. **Secrets are never stored inline.** Config structs hold only a `provider:key` reference string. The actual secret value is fetched at use time.
2. **Secrets are never logged or journaled.** `redacted_dump()` replaces ref fields before any serialization to logs or monitoring.
3. **Rotation is explicit.** Call `SecretResolver::notify_rotation(provider, key)` to invalidate any cached value. The next `resolve()` call fetches a fresh value.

## Secret reference format

```
<provider>:<key>
```

Examples:

| Reference | Provider | Key |
|-----------|----------|-----|
| `env:OPENAI_API_KEY` | env | `OPENAI_API_KEY` |
| `file:db_password` | file | `db_password` |
| `vault:secret/ancora/api_key` | vault | `secret/ancora/api_key` |

## Built-in providers

### `EnvSecretProvider`

Resolves secrets from environment variables. In tests, override with `.with_override(key, value)` to avoid real env vars.

```rust
let provider = EnvSecretProvider::new();
// or for tests:
let provider = EnvSecretProvider::new().with_override("API_KEY", "test-secret");
```

### `FileSecretProvider`

Resolves secrets from a `HashMap<String, String>` populated at startup. In production, populate from a mounted Kubernetes Secret or a Vault agent-injected file.

```rust
let mut store = HashMap::new();
store.insert("api_key".into(), std::fs::read_to_string("/run/secrets/api_key")?);
let provider = FileSecretProvider::new(store);
```

### `ExternalSecretProvider`

Adapter for external secret managers (Vault, OpenBao, AWS SSM, GCP Secret Manager). Provide a synchronous fetch closure. In production, wrap an HTTP client behind this interface.

```rust
let provider = ExternalSecretProvider::new("vault", |key| {
    // call vault API here
    vault_client.get_secret(key).map_err(|e| e.to_string())
});
```

## Registration

```rust
let mut resolver = SecretResolver::new();
resolver.register("env", Box::new(EnvSecretProvider::new()));
resolver.register("file", Box::new(FileSecretProvider::new(store)));

// Resolve at use time:
let api_key = resolver.resolve("env:OPENAI_API_KEY")?;
```

## Secret rotation

```rust
// New secret is already live in the provider backend.
// Invalidate cached value:
resolver.notify_rotation("env", "OPENAI_API_KEY")?;

// Log the rotation:
log.record("env", "OPENAI_API_KEY", now_secs);
```

After `notify_rotation`, the next `resolve()` call fetches the new value. There is no grace period: any in-flight use of the old value completes naturally.

## Kubernetes integration

Mount secrets as environment variables or files in the pod spec. Use `api_key_ref: "env:OPENAI_API_KEY"` in the Helm values:

```yaml
worker:
  apiKeyRef: "env:OPENAI_API_KEY"
```

The Helm chart renders this as an `envFrom` secret reference, keeping the actual key out of the values file and the config map.

## Audit trail

Every rotation event is recorded in `RotationLog`:

```rust
log.last_rotation_for("OPENAI_API_KEY")
// -> Some(RotationRecord { provider: "env", key: "...", rotated_at_secs: ... })
```

Persist `RotationLog` entries to the journal for compliance audit purposes.
