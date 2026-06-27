# DeepSeek Setup, Residency, and Self-Host Notes

DeepSeek offers two model families via an OpenAI-compatible API.
The direct endpoint routes traffic through CN infrastructure; a self-hosted
deployment (e.g. via vLLM) removes that constraint.

---

## Setup

### Direct endpoint

```bash
export DEEPSEEK_API_KEY=sk-...
```

The profile is registered in `providers/deepseek.rs` and uses
`https://api.deepseek.com` as the base URL.

### Self-hosted (vLLM or compatible server)

```bash
export DEEPSEEK_SELF_HOST_KEY=your-key-or-empty   # optional
```

```rust
use ancora_inference::providers::deepseek::build_deepseek_self_host_profile;

let profile = build_deepseek_self_host_profile("http://my-gpu-server:8000");
```

The self-host profile sets input/output pricing to $0 -- cost accounting
reflects no charge since the compute is user-owned.

---

## Models

| Alias | Resolved ID | Context | Tools | Input $/M | Cache $/M |
|---|---|---|---|---|---|
| `v3`, `deepseek-v3` | `deepseek-chat` | 64k | Yes | $0.27 | $0.07 |
| `r1`, `deepseek-r1` | `deepseek-reasoner` | 64k | No | $0.55 | $0.14 |
| `coder` | `deepseek-coder` | 128k | Yes | $0.14 | $0.035 |

### Cache-hit tier

When DeepSeek's KV cache is warm (the prefix has been seen before), input
tokens are billed at the lower `cache $/M` rate. The `PricingMeta.cached_per_million`
field holds this value; pass the number of cached tokens as the third argument
to `ModelMeta::compute_cost(tokens_in, tokens_out, cached_in)`.

---

## DeepSeek R1 reasoning content

DeepSeek R1 includes a `reasoning_content` field in the response message in
addition to the standard `content` field. The OpenAI client ignores unknown
fields by default (serde's `#[serde(deny_unknown_fields)]` is NOT set), so
the standard `CompletionResponse.content` is populated correctly.

If you need the reasoning chain, parse the raw response JSON:

```rust
let raw: serde_json::Value = serde_json::from_str(&raw_body)?;
let reasoning = raw["choices"][0]["message"]["reasoning_content"].as_str();
```

---

## Data residency

The direct endpoint at `api.deepseek.com` routes through CN-region infrastructure.

Use the `policy::is_allowed` function to enforce residency constraints:

```rust
use ancora_inference::policy::{is_allowed, ResidencyTag};

let excluded = vec![ResidencyTag::Cn];
if !is_allowed("deepseek", &excluded) {
    // use self-host or a different provider
}
```

| Provider name | Residency tag |
|---|---|
| `deepseek` | CN |
| `deepseek-self-host` | Unknown (user-controlled) |

---

## Self-hosted deployment with vLLM

```bash
# Start vLLM server (requires sufficient VRAM for deepseek-chat / 7B quantized)
vllm serve deepseek-ai/DeepSeek-V3 --host 0.0.0.0 --port 8000
```

Then point the self-host profile at your server:

```rust
let profile = build_deepseek_self_host_profile("http://localhost:8000");
let client = OpenAiClient::new(Arc::new(profile));
```

The server must expose an OpenAI-compatible `/v1/chat/completions` endpoint.
vLLM, llama.cpp server, and LM Studio all satisfy this requirement.

---

## Long-context RAG

DeepSeek Coder supports 128k tokens, enabling "in-context RAG" without a
vector database for small-to-medium retrieval sets:

```rust
let chunks = retrieve_top_k_documents(&query, k=50);
let context = chunks.join("\n\n---\n\n");
let messages = vec![
    Message::text("system", &format!("Context:\n{context}\nAnswer only from the context above.")),
    Message::text("user", &query),
];
let req = CompletionRequest::simple("coder", messages);
let resp = client.complete(&req)?;
```

See `crates/ancora-inference/examples/deepseek_rag.rs` for the full snippet.
