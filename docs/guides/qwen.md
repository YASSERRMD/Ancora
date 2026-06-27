# Qwen Regional Endpoints, Residency, and Self-Host Notes

Alibaba Qwen (DashScope) exposes an OpenAI-compatible API with four
international access points: Singapore, Frankfurt, Virginia, and the China
domestic endpoint. This guide covers setup, model selection, regional routing,
residency enforcement, and self-hosting open-weight Qwen models.

---

## Setup

```bash
export DASHSCOPE_API_KEY=sk-...
```

The profile is registered in `providers/qwen.rs`. It uses the Singapore
international endpoint by default.

```rust
use ancora_inference::providers::qwen::build_qwen_profile;
use ancora_inference::openai::OpenAiClient;
use std::sync::Arc;

let profile = Arc::new(build_qwen_profile());
let client = OpenAiClient::new(profile);
```

---

## Regional endpoints

DashScope provides four access points. Select a region by calling
`profile.base_url_for_region(Some("eu"))` before constructing the URL.

| Region label | Endpoint | Notes |
|---|---|---|
| `sg` | `dashscope-intl.aliyuncs.com` | Singapore; default |
| `eu` | `dashscope-intl-eu.aliyuncs.com` | Frankfurt; EU data processing |
| `us` | `dashscope-intl-us.aliyuncs.com` | Virginia; US East |
| `cn` | `dashscope.aliyuncs.com` | China domestic; CN infrastructure |

All four endpoints use the same `/compatible-mode/v1/chat/completions` path
and accept the same OpenAI-format request body.

```rust
let p = build_qwen_profile();
// Inspect which URL a region resolves to:
let eu_url = p.base_url_for_region(Some("eu"));
```

---

## Models

| Alias | Resolved ID | Context | Tools | Vision | Input $/M | Cache $/M |
|---|---|---|---|---|---|---|
| `qwen3-max` | `qwen3-235b-a22b` | 128k | Yes | No | $1.30 | -- |
| -- | `qwen3-32b` | 128k | Yes | No | $0.45 | -- |
| -- | `qwen3-14b` | 128k | Yes | No | $0.17 | -- |
| -- | `qwen3-8b` | 128k | Yes | No | $0.06 | -- |
| `qwq` | `qwq-32b` | 128k | No | No | $0.20 | -- |
| `max` | `qwen-max` | 32k | Yes | No | $1.60 | -- |
| `plus` | `qwen-plus` | 128k | Yes | No | $0.40 | $0.10 |
| `turbo` | `qwen-turbo` | 128k | Yes | No | $0.05 | $0.01 |
| `long` | `qwen-long` | 1M | No | No | $0.05 | -- |
| `vl-max` | `qwen-vl-max` | 32k | Yes | Yes | $3.00 | -- |
| `vl-plus` | `qwen-vl-plus` | 32k | No | Yes | $0.80 | -- |

### QwQ 32B (reasoning)

QwQ 32B is a reasoning model that produces chain-of-thought output before
the final answer. It does not support tool calls. Use it for complex math,
logic, and multi-step problems.

### Qwen Long (1M context)

`qwen-long` supports up to 1 million tokens -- use it for very large documents
or repository-scale code analysis where the full content fits in one request.

---

## Data residency

| Provider name (for policy) | Residency tag |
|---|---|
| `qwen` (Singapore/default) | US |
| `qwen-eu` (Frankfurt) | EU |
| `qwen-us` (Virginia) | US |
| `qwen-cn` (China domestic) | CN |
| `qwen-self-host` | Unknown (user-controlled) |

```rust
use ancora_inference::policy::{is_allowed, ResidencyTag};

// Allow only EU and US traffic
let excluded = vec![ResidencyTag::Cn];
if !is_allowed("qwen-cn", &excluded) {
    // Route to Frankfurt instead
}
```

---

## Self-hosted deployment (open-weight models)

Qwen3 32B and QwQ 32B are available under the Apache 2.0 license and can be
served via vLLM:

```bash
vllm serve Qwen/Qwen3-32B-Instruct --host 0.0.0.0 --port 8000
```

```rust
use ancora_inference::providers::qwen::build_qwen_self_host_profile;

let profile = build_qwen_self_host_profile("http://localhost:8000");
// Uses zero pricing; set QWEN_SELF_HOST_KEY if the server requires auth.
```

Self-hosted deployments use the `"qwen-self-host"` provider name in the
residency policy, which is tagged `Unknown` -- meaning residency depends on
where your GPU server runs.

---

## Multilingual usage

Qwen natively supports Chinese, Japanese, Korean, Arabic, French, Spanish,
and many other languages. No special configuration is required; just write
the prompt in the target language or ask the model to translate.

See `crates/ancora-inference/examples/qwen_multilingual.rs` for a translation
example that also shows how to inspect regional endpoint URLs.
