# Install (Rust)

## Cargo.toml

```toml
[dependencies]
ancora-core = { git = "https://github.com/ancora-ai/ancora" }
ancora-proto = { git = "https://github.com/ancora-ai/ancora" }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

For observability add:

```toml
opentelemetry = "0.21"
opentelemetry-otlp = { version = "0.14", features = ["grpc-tonic"] }
```

## Rust version

Ancora requires Rust 1.75+ (2021 edition). Check your toolchain:

```bash
rustup toolchain install stable
rustup default stable
rustc --version
```

## Feature flags

| Flag | Description |
|------|-------------|
| `sqlite` | Enable `SqliteStore` journal backend |
| `lancedb` | Enable LanceDB vector store |
| `otel` | Enable OpenTelemetry export |

Enable features selectively:

```toml
ancora-core = { git = "...", features = ["sqlite", "otel"] }
```

## Verify

```bash
cargo build
```

A successful build confirms the native Ancora library compiled for your platform.

## See also

- [Quickstart](quickstart.md)
- [Configuration](configuration.md)
