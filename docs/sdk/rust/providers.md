# Providers (Rust)

## Selecting a provider

Ancora selects the inference provider based on the model name and environment variables.

| Provider | Model prefix | Environment variable |
|----------|-------------|---------------------|
| Ollama (local) | any model without `/` | `ANCORA_MODEL_URL` (default `http://127.0.0.1:11434`) |
| Anthropic | `claude-*` | `ANTHROPIC_API_KEY` |
| OpenAI | `gpt-*` | `OPENAI_API_KEY` |
| Google Gemini | `gemini-*` | `GEMINI_API_KEY` |
| Azure OpenAI | `azure/*` | `AZURE_OPENAI_API_KEY`, `AZURE_OPENAI_ENDPOINT` |

## Ollama (local)

```rust
let spec = AgentSpec::builder()
    .model("llama3")
    .instructions("You are a helpful assistant.")
    .build();
```

Ensure Ollama is running: `ollama serve`.

## Anthropic

```bash
export ANTHROPIC_API_KEY=sk-ant-...
```

```rust
let spec = AgentSpec::builder()
    .model("claude-sonnet-4-6")
    .instructions("You are a helpful assistant.")
    .build();
```

## OpenAI

```bash
export OPENAI_API_KEY=sk-...
```

```rust
let spec = AgentSpec::builder()
    .model("gpt-4o")
    .instructions("You are a helpful assistant.")
    .build();
```

## Overriding the endpoint via `RuntimeOptions`

```rust
use ancora_core::{Runtime, RuntimeOptions};

let rt = Runtime::with_options(RuntimeOptions {
    model_url: Some("http://my-gpu-box:11434".into()),
    ..Default::default()
})?;
```

## See also

- [Chinese providers](chinese-providers.md)
- [Configuration](configuration.md)
