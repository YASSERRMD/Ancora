# Operator CRD Reference and Install Guide

The `ancora-operator` crate provides a CRD-driven operator for deploying Ancora
on Kubernetes. It manages two custom resources: `AncoraCluster` and `AncoraTenant`.

## Install

```bash
kubectl apply -f deploy/operator/ancora-cluster-crd.yaml
kubectl apply -f deploy/operator/ancora-tenant-crd.yaml
```

## AncoraCluster

Controls the full Ancora deployment including the control plane, workers, HPA,
and journal store.

```yaml
apiVersion: ancora.io/v1alpha1
kind: AncoraCluster
metadata:
  name: my-cluster
spec:
  controlPlaneReplicas: 2
  workerReplicas: 4
  workerConcurrency: 8
  autoscalerEnabled: true
  minWorkers: 2
  maxWorkers: 20
  image: "registry.example.com/ancora:v0.6.0"
  journalStore:
    backend: postgres
    connectionSecretRef: ancora-db-secret
```

## AncoraTenant

Creates an isolated tenant within an existing cluster.

```yaml
apiVersion: ancora.io/v1alpha1
kind: AncoraTenant
metadata:
  name: acme
spec:
  clusterRef: my-cluster
  maxWorkers: 5
  providerAllowlist: ["openai", "anthropic"]
  residencyRegion: uae-north
  adminRoleBinding: ancora-admin
```

## Conditions

Both resources expose `status.conditions`:

| Type | Meaning |
|------|---------|
| `Ready=True` | Resource reconciled successfully |
| `Ready=False` | Reconciliation failed or pending |
| `Degraded=True` | A dependency is unavailable |

## Upgrade procedure

See [upgrade-procedure.md](./upgrade-procedure.md) for rolling update steps
and backward-compatibility notes.
