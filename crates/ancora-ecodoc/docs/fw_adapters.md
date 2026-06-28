# Framework Adapters

Framework adapters bridge third-party orchestration frameworks into Ancora.

## Supported frameworks (adapters in progress)

- **Temporal** - Durable workflow engine
- **Airflow** - DAG-based scheduler
- **Prefect** - Modern data workflow platform
- **Custom** - Any framework via the `FrameworkAdapter` trait

## Implementing a custom adapter

```rust
use ancora_ecodoc::fw_adapters::{AdapterConfig, AdapterError, FrameworkAdapter, FrameworkKind};

pub struct MyAdapter { connected: bool }

impl FrameworkAdapter for MyAdapter {
    fn framework(&self) -> &FrameworkKind { &FrameworkKind::Custom("mine".into()) }
    fn connect(&self, _config: &AdapterConfig) -> Result<(), AdapterError> { Ok(()) }
    fn is_connected(&self) -> bool { self.connected }
}
```

Framework adapters are currently marked **unstable** and may change in minor releases.
