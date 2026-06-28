# Residency-Safe Export

ancora-obsint enforces data residency at the policy layer, preventing trace and metric
data from being sent to external or non-compliant endpoints.

## Residency Policies

### Unrestricted

Data may be exported to any endpoint. Suitable for development and cloud-native deployments.

```rust
use ancora_obsint::selfhosted::ResidencyPolicy;
let policy = ResidencyPolicy::Unrestricted;
```

### SelfHostedOnly

Data is restricted to endpoints matching one or more allowed prefixes.
Any attempt to add an external backend (Langfuse cloud, Datadog, Phoenix cloud) to
the `ExporterSelection` will return `SelectionError::ExternalBackendForbidden`.

```rust
use ancora_obsint::selfhosted::ResidencyPolicy;
let policy = ResidencyPolicy::self_hosted(vec![
    "http://10.0.0.0/8".to_string(),
    "https://obs.internal.corp".to_string(),
]);
```

## Enforcement Points

1. `ResidencyPolicy::check_endpoint` - validates a single endpoint string
2. `SelfHostedConfig::validate` - validates all configured internal endpoints
3. `ExporterSelection::add_backend` - blocks external backends under self-hosted policy
4. `is_export_permitted` - helper for runtime checks before sending data

## Compliance Use Cases

- **GDPR / data sovereignty**: keep EU customer data within EU-hosted infrastructure
- **HIPAA**: no PHI leaves the compliant network boundary
- **Air-gapped deployments**: no outbound connections required

## Audit Trail

When a residency violation is detected, a `ResidencyError` is returned (never panicked).
The caller is responsible for logging and alerting on violations.
