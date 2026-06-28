# Ancora Helm Chart: Install and Values Reference

## Prerequisites

- Helm 3.12+
- Kubernetes 1.25+
- A Secret named `ancora-auth-secret` with key `token`

## Install

```bash
helm install ancora deploy/helm/ancora \
  --namespace ancora --create-namespace \
  --set auth.tokenSecretRef=ancora-auth-secret
```

## Upgrade

```bash
helm upgrade ancora deploy/helm/ancora --namespace ancora
```

## Uninstall

```bash
helm uninstall ancora --namespace ancora
```

## Values reference

| Key | Default | Description |
|-----|---------|-------------|
| `global.mode` | `single-tenant` | Deployment mode: `single-tenant` or `multi-tenant` |
| `global.registry` | `""` | Optional image registry prefix |
| `image.repository` | `ancora` | Image name |
| `image.tag` | `v0.6.0` | Image tag |
| `image.pullPolicy` | `IfNotPresent` | Pull policy |
| `controlPlane.replicaCount` | `1` | Control plane replicas |
| `controlPlane.service.port` | `8080` | Control plane HTTP port |
| `worker.replicaCount` | `2` | Worker replicas |
| `worker.concurrency` | `4` | Concurrent runs per worker |
| `autoscaling.enabled` | `false` | Enable HPA for workers |
| `autoscaling.minReplicas` | `2` | HPA minimum replicas |
| `autoscaling.maxReplicas` | `10` | HPA maximum replicas |
| `journalStore.backend` | `sqlite` | Journal backend: `sqlite` or `postgres` |
| `journalStore.sqlite.path` | `/data/ancora.db` | SQLite database path |
| `journalStore.postgres.connectionSecretRef` | - | Secret name for PostgreSQL connection string |
| `ingress.enabled` | `false` | Enable Ingress |
| `ingress.className` | `nginx` | IngressClass name |
| `networkPolicy.enabled` | `true` | Enable NetworkPolicy |
| `auth.tokenSecretRef` | `ancora-auth-secret` | Secret name for the auth token |
| `airgap.enabled` | `false` | Enable air-gap mode |
| `airgap.registry` | - | Private registry host |
| `airgap.pullSecretName` | `regcred` | Image pull secret name |

## Profiles

### Single-tenant (default)

```bash
helm install ancora deploy/helm/ancora
```

### Multi-tenant

```bash
helm install ancora deploy/helm/ancora \
  -f deploy/helm/ancora/values-multi-tenant.yaml
```

### Air-gapped

```bash
helm install ancora deploy/helm/ancora \
  -f deploy/helm/ancora/values-airgap.yaml
```
