# OpenAI, Azure OpenAI, and OpenRouter setup

## OpenAI

```rust
use ancora_inference::providers::openai::build_openai_profile;
use ancora_inference::provider::ProviderRegistry;

let mut registry = ProviderRegistry::new();
registry.register(build_openai_profile());
```

Set your key before running:

```bash
export OPENAI_API_KEY=sk-...
```

**Included models:** `gpt-4o`, `gpt-4o-mini`, `o1`, `o3-mini`, `o4-mini`.
Aliases: `gpt-4o-latest` -> `gpt-4o`, `o3` -> `o3-mini`.

Tool-calling and vision are enabled on `gpt-4o` and `gpt-4o-mini`. o-series models support tools but not vision.

## Azure OpenAI

```rust
use ancora_inference::providers::azure::build_azure_profile;

let profile = build_azure_profile(
    "my-resource",       // Azure resource name
    "gpt-4o-deploy",     // deployment name
    "2024-02-01",        // api-version
);
registry.register(profile);
```

Set your key before running:

```bash
export AZURE_OPENAI_API_KEY=...
```

**What Azure does differently:**
- URL is `https://{resource}.openai.azure.com/openai/deployments/{deployment}/chat/completions?api-version=...`
- Auth uses `api-key: <value>` header (not `Authorization: Bearer`)
- The `model` field is dropped from the JSON body (deployment name encodes the model)

## OpenRouter

OpenRouter routes to any supported provider using `provider/model` namespacing.

```rust
use ancora_inference::providers::openrouter::{build_openrouter_profile, OpenRouterConfig};

let profile = build_openrouter_profile(OpenRouterConfig {
    model_id: "openai/gpt-4o".to_string(),
    fallback_models: vec!["anthropic/claude-3-5-haiku".to_string()],
    app_name: "MyApp".to_string(),
    site_url: "https://myapp.example".to_string(),
});
registry.register(profile);
```

Set your key before running:

```bash
export OPENROUTER_API_KEY=sk-or-...
```

**Attribution headers:** `HTTP-Referer` and `X-Title` are added automatically from the config.

**Fallback models:** when set, injected as the `models` array in the request body.
OpenRouter tries models in order when the primary is unavailable.

## Usage accounting

```rust
use ancora_inference::providers::usage::UsageSummary;

let resp = client.complete(&request)?;
let summary = UsageSummary::from_response("openai", "gpt-4o", &resp);
println!("cost: {:?} USD", summary.cost_usd);
```

`cost_usd` is `Some(f64)` when the profile has pricing metadata for the model,
`None` for Azure deployments (which have no pricing in the profile) and custom endpoints.
