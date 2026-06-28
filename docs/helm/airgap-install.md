# Air-Gapped Helm Install with Private Registry

## Overview

In air-gapped environments no public internet access is available.
All images must be pre-loaded into an internal container registry.

## Steps

### 1. Mirror the image

```bash
docker pull ancora:v0.6.0
docker tag ancora:v0.6.0 registry.internal.example.com/ancora/ancora:v0.6.0
docker push registry.internal.example.com/ancora/ancora:v0.6.0
```

### 2. Create pull secret

```bash
kubectl create secret docker-registry regcred \
  --docker-server=registry.internal.example.com \
  --docker-username=<user> \
  --docker-password=<pass> \
  -n ancora
```

### 3. Create auth token secret

```bash
kubectl create secret generic ancora-auth-secret \
  --from-literal=token=<your-token> \
  -n ancora
```

### 4. Install with air-gap values

```bash
helm install ancora deploy/helm/ancora \
  --namespace ancora --create-namespace \
  -f deploy/helm/ancora/values-airgap.yaml \
  --set airgap.registry=registry.internal.example.com \
  --set airgap.pullSecretName=regcred
```

## Verification

```bash
kubectl rollout status deployment/ancora-control-plane -n ancora
kubectl rollout status deployment/ancora-worker -n ancora
```

No outbound network calls are made by Ancora itself. Provider API keys are
optional and only used when a run explicitly calls an LLM provider.
