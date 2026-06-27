# Chinese Providers (Rust)

## DeepSeek

```bash
export DEEPSEEK_API_KEY=sk-...
```

```rust
let spec = AgentSpec::builder()
    .model("deepseek-chat")
    .instructions("You are a helpful assistant.")
    .build();
```

Endpoint: `https://api.deepseek.com`

## Zhipu GLM

```bash
export GLM_API_KEY=...
```

```rust
let spec = AgentSpec::builder()
    .model("glm-4")
    .instructions("You are a helpful assistant.")
    .build();
```

Endpoint: `https://open.bigmodel.cn/api/paas/v4`

## Alibaba Qwen (global)

```bash
export DASHSCOPE_API_KEY=sk-...
```

```rust
let spec = AgentSpec::builder()
    .model("qwen-plus")
    .instructions("You are a helpful assistant.")
    .build();
```

Endpoint: `https://dashscope.aliyuncs.com/compatible-mode/v1`

## Qwen -- Singapore region

For data-residency in Singapore, override the endpoint:

```rust
use ancora_core::{Runtime, RuntimeOptions};

let rt = Runtime::with_options(RuntimeOptions {
    model_url: Some("https://dashscope-intl.aliyuncs.com/compatible-mode/v1".into()),
    ..Default::default()
})?;
```

## Air-gapped / private deployment

All Chinese providers expose an OpenAI-compatible chat endpoint.
Point `model_url` at any such endpoint:

```rust
let rt = Runtime::with_options(RuntimeOptions {
    model_url: Some("http://10.0.0.5:8080/v1".into()),
    ..Default::default()
})?;
```

## See also

- [Providers](providers.md)
- [Policy](policy.md)
