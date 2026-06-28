# Ancora Packaging Guide

ancora-pkg provides deployment templates for all Ancora product delivery modes.

## Supported Deployment Modes

| Mode | Template | Use Case |
|------|----------|----------|
| SaaS | `saas_template` | Multi-tenant public cloud |
| On-Prem | `onprem_template` | Customer-hosted appliance |
| Air-Gapped | `airgap_template` | No external network access |
| Compose | `compose_template` | Local dev / single host |
| Kubernetes | `k8s_template` | Cloud-native cluster |
| Edge | `edge_template` | IoT / edge single binary |
| White-Label | `whitelabel` | OEM / partner rebranding |
| Tenant Onboard | `tenant_onboard` | Multi-tenant provisioning |

## Quickstart

Use the `PackagingCli::scaffold` function to generate a template:

```rust
use ancora_pkg::cli::{PackagingCli, ScaffoldArgs, ScaffoldKind};

let args = ScaffoldArgs::new(ScaffoldKind::Saas, "my-product")
    .with_output("./deploy")
    .with_extra("region", "us-east-1");

let output = PackagingCli::scaffold(args).unwrap();
for file in &output.files {
    println!("Generated: {}", file.path);
}
```

## Secure Defaults

Every template enforces secure defaults out of the box:

- TLS 1.3 minimum
- Non-root container execution
- Read-only root filesystem
- MFA required
- Audit logging enabled
- Network policies applied

Secure defaults cannot be disabled through the template API.
