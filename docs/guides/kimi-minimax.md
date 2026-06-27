# Kimi and MiniMax Setup Notes

Both Moonshot Kimi and MiniMax expose OpenAI-compatible chat-completion APIs
and specialize in very long context windows (1M+ tokens) and multimodal
capabilities. Neither has open-weight models; use a gateway profile for
private-network routing.

---

## Kimi (Moonshot AI)

### Setup

```bash
export MOONSHOT_API_KEY=your_api_key_here
```

```rust
use ancora_inference::providers::kimi::build_kimi_profile;

let profile = build_kimi_profile();  // defaults to api.moonshot.ai (international)
```

### Endpoints

| Label | Endpoint | Notes |
|---|---|---|
| `intl` (default) | `api.moonshot.ai` | International; non-CN routing |
| `cn` | `api.moonshot.cn` | China domestic |

```rust
let url = profile.base_url_for_region(Some("cn"));
```

The `kimi-cn` profile (`build_kimi_domestic_profile()`) uses the CN endpoint
with the same API key and is tagged with `ResidencyTag::Cn` in the policy layer.

### Models

| Alias | Resolved ID | Context | Tools | Input $/M |
|---|---|---|---|---|
| `k2` | `kimi-k2` | 128k | Yes | $0.60 |
| `k2-turbo` | `kimi-k2-turbo` | 128k | Yes | $0.20 |
| `128k` | `moonshot-v1-128k` | 128k | No | $1.00 |
| `32k` | `moonshot-v1-32k` | 32k | No | $0.24 |
| `8k` | `moonshot-v1-8k` | 8k | No | $0.12 |
| `long` | `moonshot-v1-long` | 1M | No | $1.50 |

### Gateway profile (private network)

Kimi K2 is not open-weight. To route through an on-premises gateway:

```rust
use ancora_inference::providers::kimi::build_kimi_gateway_profile;

let profile = build_kimi_gateway_profile("http://litellm.internal:4000");
// Uses KIMI_GATEWAY_KEY for auth; zero pricing (gateway handles billing)
```

### Residency

| Provider name | Tag |
|---|---|
| `kimi` | US (international) |
| `kimi-cn` | CN |
| `kimi-gateway` | Unknown |

---

## MiniMax

### Setup

```bash
export MINIMAX_API_KEY=your_api_key_here
```

```rust
use ancora_inference::providers::minimax::build_minimax_profile;

let profile = build_minimax_profile();
```

### Models

| Alias | Resolved ID | Context | Tools | Vision | Input $/M |
|---|---|---|---|---|---|
| `text-01` | `MiniMax-Text-01` | 1M | Yes | No | $0.20 |
| `vl-01` | `MiniMax-VL-01` | 1M | No | Yes | $0.80 |
| `m2` | `MiniMax-M2` | 128k | Yes | No | $0.15 |
| `speech` | `MiniMax-Speech-02-Turbo` | -- | No | No | $0.008/char |

### Multimodal

`MiniMax-VL-01` accepts image content via the standard OpenAI vision format
(base64-encoded images in the `content` array). Use `is_vision_model()` to
check capability at runtime:

```rust
use ancora_inference::providers::minimax::is_vision_model;
assert!(is_vision_model("vl-01"));
```

Speech/audio (`MiniMax-Speech-02-Turbo`) uses a different endpoint
(`/v1/t2a_v2`) and is not chat-completion compatible. Use `is_speech_model()`
to detect these cases and route appropriately.

### Streaming

```rust
use ancora_inference::providers::minimax::{collect_stream_text, parse_stream_line};

let full_text = collect_stream_text(&sse_lines);
```

### Residency

MiniMax routes through CN infrastructure. Tag: `ResidencyTag::Cn`.

```rust
use ancora_inference::policy::{is_allowed, ResidencyTag};

let excluded = vec![ResidencyTag::Cn];
assert!(!is_allowed("minimax", &excluded));
```
