# Configuration (Rust)

## Environment variables

| Variable | Default | Description |
|----------|---------|-------------|
| `ANCORA_MODEL_URL` | `http://127.0.0.1:11434` | Inference endpoint URL |
| `RUST_LOG` | (unset) | Tracing filter (`ancora=debug`, `ancora=trace`) |
| `ANTHROPIC_API_KEY` | (none) | API key for Anthropic endpoints |
| `OPENAI_API_KEY` | (none) | API key for OpenAI endpoints |
| `GLM_API_KEY` | (none) | API key for Zhipu GLM |
| `DASHSCOPE_API_KEY` | (none) | API key for Alibaba Qwen |
| `DEEPSEEK_API_KEY` | (none) | API key for DeepSeek |

## `RuntimeOptions`

```rust
use ancora_core::{Runtime, RuntimeOptions, SqliteStore, StoringTransport};
use std::time::Duration;

let rt = Runtime::with_options(RuntimeOptions {
    model_url: Some("http://127.0.0.1:11434".into()),
    transport: Some(Box::new(StoringTransport::new(SqliteStore::open("journal.db")?))),
    http_timeout: Some(Duration::from_secs(300)),
    tracer: None,
})?;
```

## Reading from a config file with `config` crate

```toml
# config/default.toml
model_url = "http://127.0.0.1:11434"
model = "llama3"
```

```rust
use config::{Config, File};

let settings = Config::builder()
    .add_source(File::with_name("config/default"))
    .add_source(config::Environment::with_prefix("ANCORA"))
    .build()?;

let model_url: String = settings.get_string("model_url")?;
let model: String = settings.get_string("model")?;

let rt = Runtime::with_options(RuntimeOptions {
    model_url: Some(model_url),
    ..Default::default()
})?;
```

## Logging with `tracing-subscriber`

```rust
tracing_subscriber::fmt()
    .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
    .init();
```

Run with:

```bash
RUST_LOG=ancora=debug cargo run
```

## See also

- [Providers](providers.md)
- [Install](install.md)
