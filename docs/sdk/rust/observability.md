# Observability (Rust)

## Cost tracking from `RunEvent::Completed`

```rust
use ancora_core::RunEvent;

while let Some(ev) = run.next().await? {
    if let RunEvent::Completed { output, usage, .. } = ev {
        println!("Output   : {}", output);
        println!("In tokens: {}", usage.input_tokens);
        println!("Out tokens: {}", usage.output_tokens);
        println!("Cost (USD): {:.6}", usage.cost_usd());
    }
}
```

## OpenTelemetry span export

```toml
opentelemetry = "0.21"
opentelemetry-otlp = { version = "0.14", features = ["grpc-tonic"] }
opentelemetry_sdk = { version = "0.21", features = ["rt-tokio"] }
ancora-core = { git = "...", features = ["otel"] }
```

```rust
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{runtime, trace::Config};

let exporter = opentelemetry_otlp::new_exporter()
    .tonic()
    .with_endpoint("http://localhost:4317");

let tracer = opentelemetry_otlp::new_pipeline()
    .tracing()
    .with_exporter(exporter)
    .with_trace_config(Config::default())
    .install_batch(runtime::Tokio)?;
```

Configure Ancora to use the tracer:

```rust
use ancora_core::{Runtime, RuntimeOptions};

let rt = Runtime::with_options(RuntimeOptions {
    tracer: Some(tracer),
    ..Default::default()
})?;
```

Each run emits a root span named `ancora.run` with child spans per tool call.

## Logging with `tracing`

Ancora emits `tracing` events at `debug` and `trace` level.
Install a subscriber to capture them:

```bash
RUST_LOG=ancora=debug cargo run
```

```rust
tracing_subscriber::fmt::init();
```

## See also

- [Configuration](configuration.md)
- [Durability](durability.md)
