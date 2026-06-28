# SaaS vs On-Prem vs Air-Gapped

Choosing the right deployment mode depends on your customer's connectivity,
compliance, and operational requirements.

## SaaS

- Runs on Ancora-managed cloud infrastructure
- Multi-tenant by default with namespace isolation
- Automatic updates and patching
- Customer data stays in selected cloud region
- Suitable for: most commercial customers

## On-Prem

- Runs on customer-managed servers or private cloud
- Single-tenant per installation
- Customer controls update cadence
- Supports HSM and local keyring for secrets
- Suitable for: regulated enterprises, financial services

## Air-Gapped

- No external network connectivity permitted
- All artifacts must be bundled locally (registry mirror or tar bundle)
- Offline license file required
- FIPS mode enabled by default
- Suitable for: government, defence, critical infrastructure

## Decision Matrix

| Requirement | SaaS | On-Prem | Air-Gapped |
|-------------|------|---------|------------|
| Managed updates | Yes | Optional | No |
| Internet access required | Yes | Optional | No |
| FIPS compliance | Optional | Optional | Yes |
| Customer data on-site | No | Yes | Yes |
| HSM support | No | Yes | Yes |
| Multi-tenant | Yes | No | No |

## Migration Path

Customers can start on SaaS and migrate to On-Prem or Air-Gapped using the
same Ancora agent binary - only the deployment template changes.
