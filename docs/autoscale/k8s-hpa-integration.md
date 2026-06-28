# Scale-Signal Integration for Kubernetes HPA

Ancora's autoscaler emits `ScaleSignal` JSON objects. An adapter sidecar
reads these signals and exposes `desired_workers` as a custom metric in the
Kubernetes custom-metrics API.

## Flow

```
Ancora autoscaler
      |
      | ScaleSignal (JSON, stdout or Unix socket)
      v
  Adapter sidecar
      |
      | custom-metrics-apiserver registration
      v
  Kubernetes HPA
      |
      | scales Deployment/StatefulSet replicas
      v
  Ancora Worker Deployment
```

## Minimal adapter pattern

```yaml
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: ancora-workers
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: ancora-worker
  minReplicas: 2
  maxReplicas: 20
  metrics:
    - type: External
      external:
        metric:
          name: ancora_desired_workers
        target:
          type: AverageValue
          averageValue: "1"
```

## Notes

- The adapter should de-duplicate signals within the HPA polling interval.
- Set `minReplicas` to match `ScaleBounds.min_workers` for consistency.
- The HPA converges within one polling period (default 15s).
