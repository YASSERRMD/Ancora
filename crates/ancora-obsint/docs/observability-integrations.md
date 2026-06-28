# Observability Integrations Guide

ancora-obsint provides traces and metrics export to common observability backends.

## Supported Backends

| Backend | Type | Self-hostable |
|---------|------|---------------|
| OTLP gRPC/HTTP | Traces + Metrics | Yes |
| Langfuse | Traces (LLM) | Yes (OSS) |
| Phoenix (Arize) | Traces (LLM) | Yes (OSS) |
| Grafana Tempo | Traces | Yes |
| Grafana Loki | Logs | Yes |
| Datadog APM | Traces + Metrics | No |
| Prometheus | Metrics (scrape) | Yes |

## Quick Start

Configure exporters via `ExporterSelection`. Add backends that match your residency policy.

```rust
use ancora_obsint::selection::{ExporterSelection, ExporterBackend, parse_backends};
use ancora_obsint::selfhosted::ResidencyPolicy;

let policy = ResidencyPolicy::Unrestricted;
let mut sel = ExporterSelection::new(policy);
sel.add_backend(ExporterBackend::Otlp).unwrap();
sel.add_backend(ExporterBackend::Prometheus).unwrap();
```

## OTLP Export

Ancora emits OpenTelemetry-compatible spans and metrics via the `otlp` module.
Both gRPC (default port 4317) and HTTP/protobuf (default port 4318) are supported.

## Health Checks

Use `HealthChecker` to monitor exporter connectivity. `summarize_health` rolls up
individual reports into an overall system status.
