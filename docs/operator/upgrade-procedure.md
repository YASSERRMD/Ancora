# Operator Upgrade Procedure

## Rolling update

1. Update the `image` field in the `AncoraCluster` spec.
2. The operator triggers a rolling update on the worker `Deployment`.
3. The control plane replica is updated first, then workers.
4. Monitor `status.conditions` for `Ready=True`.

## Schema migration

When upgrading between minor versions:

1. Apply the new CRD YAML: `kubectl apply -f deploy/operator/ancora-cluster-crd.yaml`.
2. The operator will add any new optional fields with defaults on next reconcile.

## Backward compatibility

The operator reads all `v1alpha1` CRs. Any field added in a new version must
have a serde default so older CRs remain valid.

## Rollback

Set `image` back to the previous version in the `AncoraCluster` spec.
The operator performs a rolling rollback automatically.
