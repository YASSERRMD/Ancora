# Self-Hosted Observability Stack

Run a complete observability stack on your own infrastructure with zero data leaving your network.

## Recommended Stack

- **Grafana Tempo** - distributed tracing storage and query
- **Grafana Loki** - log aggregation
- **Prometheus** - metrics collection via scrape
- **Grafana** - dashboards and alerting UI
- **OpenTelemetry Collector** - OTLP receiver and fan-out

## Configuration

Set the residency policy to `SelfHostedOnly` with your internal network prefixes:

```rust
use ancora_obsint::selfhosted::{ResidencyPolicy, SelfHostedConfig};

let policy = ResidencyPolicy::self_hosted(vec![
    "http://tempo.internal:3200".to_string(),
    "http://loki.internal:3100".to_string(),
    "http://otel-collector.internal:4317".to_string(),
]);

let cfg = SelfHostedConfig::new(policy)
    .with_tempo("http://tempo.internal:3200")
    .with_loki("http://loki.internal:3100")
    .with_otlp("http://otel-collector.internal:4317");

cfg.validate().expect("all endpoints within policy");
```

## Data Flow

```
Ancora agents
     |
     v
OTLP Collector (internal)
     |
     +---> Tempo (traces)
     |
     +---> Loki (logs)
     |
     +---> Prometheus (metrics scrape)
     |
     v
Grafana (dashboards)
```

## Security

- All traffic stays within your private network
- No API keys sent to external services
- Mutual TLS recommended between components
