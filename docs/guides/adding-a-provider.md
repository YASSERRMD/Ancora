# Adding a new provider

Every provider in Ancora is a `ProviderProfile`. No new client type is needed. Adding a provider means registering a profile.

## Minimal example

```rust
use ancora_inference::provider::{AuthStrategy, ModelMeta, ProviderProfile, ProviderRegistry};

fn register_acme(registry: &mut ProviderRegistry) {
    let profile = ProviderProfile::new(
        "acme",                        // registry key
        "https://api.acme.example",    // base URL
        AuthStrategy::BearerToken { env_var: "ACME_API_KEY".to_string() },
    )
    .add_model(
        ModelMeta::new("acme-large", 128_000)
            .with_pricing(5.0, 15.0)   // USD per million tokens: input, output
            .with_tools()
            .with_streaming(),
    )
    .add_alias("large", "acme-large"); // short name for convenience

    registry.register(profile);
}
```

Set the env var before running:

```bash
export ACME_API_KEY=sk-...
```

## Auth strategies

| Strategy | Use when |
|---|---|
| `BearerToken { env_var }` | Provider uses `Authorization: Bearer <token>` (OpenAI, Anthropic, most) |
| `HeaderKey { header, env_var }` | Provider uses a custom header name (some Chinese providers, Replicate) |
| `QueryParam { param, env_var }` | Provider appends the key as a URL parameter |
| `None` | Local server with no auth (Ollama, llama.cpp, vLLM on localhost) |

## Regional endpoints

```rust
let profile = ProviderProfile::new("qwen", "https://dashscope.aliyuncs.com", auth)
    .add_region("eu", "https://dashscope.eu.aliyuncs.com")
    .add_region("us", "https://dashscope.us.aliyuncs.com");
```

Select a region when constructing the client:

```rust
let client = OpenAiClient::new(Arc::new(profile)).with_region("eu");
```

## Request and response transforms

Providers that diverge slightly from the OpenAI shape can be adapted with transforms:

```rust
use ancora_inference::provider::transform::set_field;

let profile = ProviderProfile::new("special", "https://api.special.ai", auth)
    .with_request_transform(set_field("safe_mode", serde_json::json!(true)))
    .with_request_transform(|body| {
        // rename a field
        if let Some(v) = body.get("max_tokens").cloned() {
            body["max_completion_tokens"] = v;
            body.as_object_mut().unwrap().remove("max_tokens");
        }
    });
```

Transforms are applied in registration order, before the HTTP call for requests
and after for responses.

## Verifying your provider works

Write a unit test using a recorded fixture (no live key needed):

```rust
#[test]
fn acme_recorded_fixture_completes() {
    const FIXTURE: &str = r#"{"choices":[{"message":{"role":"assistant","content":"ok"},"finish_reason":"stop"}],"usage":{"prompt_tokens":5,"completion_tokens":2}}"#;
    let profile = Arc::new(
        ProviderProfile::new("acme", "http://localhost", AuthStrategy::None)
            .add_model(ModelMeta::new("acme-large", 128_000).with_pricing(5.0, 15.0)),
    );
    let client = OpenAiClient::new(profile);
    let resp = client.parse_response(FIXTURE, "acme-large").unwrap();
    assert_eq!(resp.content, "ok");
    assert!(resp.cost_usd.is_some());
}
```

## Checklist

- [ ] Create a `ProviderProfile` with name, base URL, and auth strategy
- [ ] Add model metadata (context window, pricing, capabilities)
- [ ] Add aliases for common short names
- [ ] Add regional base-URL overrides if the provider supports them
- [ ] Add request/response transforms for any shape differences
- [ ] Write a recorded-fixture test (no live key)
- [ ] Register the profile in the application-level `ProviderRegistry`
