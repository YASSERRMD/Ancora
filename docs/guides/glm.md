# GLM Setup, Self-Host, and License Notes

Zhipu AI's GLM (General Language Model) series is accessible via `open.bigmodel.cn`
with an OpenAI-compatible API. GLM-4-9B is released under the MIT license and can
be self-hosted via vLLM (GPU) or llama.cpp (CPU/edge).

---

## Setup

```bash
export ZHIPU_API_KEY=your_api_key_here
```

The profile is in `providers/glm.rs`. Note the non-standard API path:
`open.bigmodel.cn/api/paas/v4/chat/completions`.

```rust
use ancora_inference::providers::glm::build_glm_profile;
use ancora_inference::openai::OpenAiClient;
use std::sync::Arc;

let profile = Arc::new(build_glm_profile());
let client = OpenAiClient::new(profile);
```

---

## Models

| Alias | Resolved ID | Context | Tools | Vision | Input $/M |
|---|---|---|---|---|---|
| `glm5` | `glm-5` | 128k | Yes | No | $0.60 |
| `glm5.1` | `glm-5.1` | 128k | Yes | No | $0.80 |
| -- | `glm-5-long` | 256k | No | No | $0.60 |
| `turbo` | `glm-turbo` | 128k | Yes | No | $0.06 |
| `flash` | `glm-4-flash` | 128k | No | No | $0.01 |
| `vl` | `glm-4v` | 8k | No | Yes | $0.25 |

---

## Structured output (JSON mode)

GLM supports `response_format: {"type": "json_object"}` to force a JSON
object response. Use `build_glm_json_profile()` to inject this automatically:

```rust
use ancora_inference::providers::glm::{build_glm_json_profile, is_json_object};

let profile = Arc::new(build_glm_json_profile());
let client = OpenAiClient::new(profile);
// Every request will have response_format.type = "json_object" injected
let resp = client.complete(&req)?;
assert!(is_json_object(&resp.content));
```

See `crates/ancora-inference/examples/glm_extraction.rs` for a complete
entity-extraction snippet.

---

## Data residency

`open.bigmodel.cn` routes through Chinese infrastructure.

```rust
use ancora_inference::policy::{is_allowed, ResidencyTag};

let excluded = vec![ResidencyTag::Cn];
if !is_allowed("glm", &excluded) {
    // Use self-host instead
}
```

| Provider name | Residency |
|---|---|
| `glm` | CN |
| `glm-self-host` | Unknown (user-controlled) |
| `glm-llamacpp` | Unknown (user-controlled) |

---

## Self-hosted deployment

### vLLM (GPU -- recommended)

GLM-4-9B-Chat is MIT-licensed and can run on a single A100 or two A6000s.

```bash
vllm serve THUDM/glm-4-9b-chat --host 0.0.0.0 --port 8000
```

```rust
use ancora_inference::providers::glm::build_glm_self_host_profile;

let profile = build_glm_self_host_profile("http://localhost:8000");
// profile.name = "glm-self-host", zero pricing, reads GLM_SELF_HOST_KEY
```

### llama.cpp (CPU / edge)

For edge deployments, llama.cpp can serve a quantized GLM-4-9B GGUF model
using as little as 5 GB of RAM at 4-bit precision.

```bash
./llama-server -m glm-4-9b-chat-q4_k_m.gguf --port 8080
```

```rust
use ancora_inference::providers::glm::build_glm_llamacpp_profile;

let profile = build_glm_llamacpp_profile("http://localhost:8080");
// profile.name = "glm-llamacpp", AuthStrategy::None, zero pricing
```

---

## License

| Model | License |
|---|---|
| GLM-5 / GLM-5.1 | Proprietary (Zhipu AI) |
| GLM-4-9B / GLM-4-9B-Chat | MIT |
| GLM-4V | Proprietary |
| GLM Turbo | Proprietary |

The open-weight MIT variants (GLM-4-9B family) can be used commercially
and self-hosted without restriction.
