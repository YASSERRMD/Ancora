# Gateway Transports

Ancora ships with four gateway transports in addition to direct provider integrations.
Gateways act as routing layers, letting you target many underlying models through a
single endpoint and add cross-cutting features like fallbacks, caching, and observability.

## Provider Summary

| Gateway | Module | Auth | Residency | Key feature |
|---------|--------|------|-----------|-------------|
| OpenRouter | `openrouter` | `OPENROUTER_API_KEY` | US | Multi-provider fallback |
| LiteLLM | `litellm` | `LITELLM_API_KEY` | Unknown (self-host) | Local proxy, 100+ providers |
| Portkey | `portkey` | `PORTKEY_API_KEY` (header) | US | Enterprise routing / caching |
| Vercel AI Gateway | `vercelai` | `VERCEL_AI_TOKEN` | US | Managed gateway on Vercel infra |

## OpenRouter

Routes to any of 200+ models using `provider/model` namespaced IDs. Supports model
fallbacks in a single request via the `models` field.

```rust
use ancora_inference::providers::openrouter::{build_openrouter_profile, OpenRouterConfig};

let profile = build_openrouter_profile(OpenRouterConfig {
    model_id: "openai/gpt-4o".to_string(),
    fallback_models: vec!["anthropic/claude-3-5-haiku".to_string()],
    app_name: "MyApp".to_string(),
    site_url: "https://myapp.example".to_string(),
});

// Or with a single model:
use ancora_inference::providers::openrouter::build_openrouter_simple;
let profile = build_openrouter_simple("anthropic/claude-3-7-sonnet");
```

## LiteLLM

Self-hosted proxy that translates any OpenAI-compatible request to 100+ providers.
Run it locally with `pip install litellm && litellm --model gpt-4o`.

```rust
use ancora_inference::providers::litellm::{build_litellm_profile, build_litellm_noauth_profile, LITELLM_DEFAULT_URL};

// With API key auth (controlled via LITELLM_API_KEY):
let profile = build_litellm_profile("http://localhost:4000");

// For purely local deployments with no auth:
let profile = build_litellm_noauth_profile(LITELLM_DEFAULT_URL);

// With routing tags for cost attribution:
use ancora_inference::providers::litellm::build_litellm_tagged_profile;
let profile = build_litellm_tagged_profile("http://localhost:4000", &["prod", "team-a"]);
```

Model IDs use `provider/model` format:

```
openai/gpt-4o
anthropic/claude-3-5-haiku-20241022
gemini/gemini-2.0-flash
mistral/mistral-small-latest
groq/llama3-8b-8192
together_ai/meta-llama/Llama-3-8b-chat-hf
```

## Portkey

Enterprise AI gateway with routing, retries, caching, load balancing, and
observability. Uses `x-portkey-api-key` header auth instead of a standard
Bearer token.

```rust
use ancora_inference::providers::portkey::{build_portkey_profile, build_portkey_from_config, PortkeyConfig};

// Simple profile:
let profile = build_portkey_profile();

// With dashboard config and trace ID:
let cfg = PortkeyConfig::new()
    .with_config("my-routing-config")
    .with_trace_id("req-abc123");
let profile = build_portkey_from_config(cfg);
```

Portkey's `x-portkey-config` header references a routing strategy you configure in
the Portkey dashboard (retries, load balancing, fallbacks, caching).

For virtual keys (pre-configured provider credentials):

```rust
use ancora_inference::providers::portkey::build_portkey_virtual_key_profile;

// PORTKEY_VK is the env var holding your virtual key
let profile = build_portkey_virtual_key_profile("PORTKEY_VK");
```

## Vercel AI Gateway

Managed gateway on Vercel infrastructure. Routes to OpenAI, Anthropic, Mistral,
and other providers using `provider/model` IDs.

```rust
use ancora_inference::providers::vercelai::{build_vercelai_profile, extract_provider};

let profile = build_vercelai_profile();

// Detect which upstream provider a routed model ID targets:
let provider = extract_provider("anthropic/claude-3-7-sonnet");
assert_eq!(provider, Some("anthropic"));
```

## Fallback Chaining

Use `FallbackChain` to build provider-agnostic ordered fallback sequences:

```rust
use ancora_inference::providers::gateway::FallbackChain;

let mut chain = FallbackChain::new()
    .push("openai", "gpt-4o")
    .push("anthropic", "claude-3-5-haiku")
    .push("mistral", "mistral-small-latest");

let (provider, model) = chain.primary().unwrap();
// Try provider/model ...

// On failure, advance to the next fallback:
if let Some((provider, model)) = chain.next_fallback() {
    // Try the fallback ...
}

// Build OpenRouter-compatible models list from the chain:
let models = chain.to_openrouter_models();
// ["openai/gpt-4o", "anthropic/claude-3-5-haiku", "mistral/mistral-small-latest"]
```

## Cost Headers

OpenRouter reports the actual request cost in a response header:

```rust
use ancora_inference::providers::gateway::parse_openrouter_cost_header;

// Assuming `response_headers` is a HashMap<String, String>:
let cost = parse_openrouter_cost_header(response_headers.get("x-openrouter-cost").map(|s| s.as_str()));
// cost: Some(0.000042) or None
```

## Residency

| Provider | Residency |
|----------|-----------|
| openrouter | US |
| litellm | Unknown (self-hosted; you control residency) |
| litellm-local | Unknown |
| portkey | US |
| vercelai | US |

LiteLLM is tagged `Unknown` because the data residency of requests depends
on where you run the proxy and which upstream provider it routes to.
