# Self-Hosted Observability Summary

## Supported Backends

| Backend | Metrics | Traces | Logs | Status |
| --- | --- | --- | --- | --- |
| Prometheus + Thanos | Yes | No | No | GA |
| Victoria Metrics | Yes | No | No | GA |
| Grafana Tempo | No | Yes | No | GA |
| OpenSearch | No | No | Yes | Beta |
| Loki | No | No | Yes | Beta |
| Grafana LGTM stack | Yes | Yes | Yes | GA |

## Supported Topologies

- **Single-node**: development and small teams. Not production-grade.
- **High-availability**: >= 2 replicas, recommended for production.
- **Federated multi-region**: Thanos-based global view across regions.

## Minimum Requirements (HA topology)

| Component | vCPU | RAM | Storage |
| --- | --- | --- | --- |
| Metrics backend | 2 | 4 GB | 100 GB SSD |
| Trace backend | 2 | 4 GB | 200 GB SSD |
| Log backend | 2 | 4 GB | 500 GB HDD |
| Collector (per node) | 0.5 | 512 MB | - |

## Security Requirements for Production Grade

- TLS enforced on all inter-component links
- Auth provider configured (OIDC or LDAP)
- Topology must be HA or federated-multi-region

## Quick Start

```bash
# Deploy the LGTM stack via Helm
helm repo add ancora https://charts.ancora.dev
helm install ancora-obs ancora/obs-stack --set topology=ha
```

Last updated: 2026-06-29
