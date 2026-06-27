# Chinese Lab Providers

This guide covers all nine Chinese-lab providers that Ancora ships with:
DeepSeek, Qwen, GLM, Kimi, MiniMax, StepFun, ERNIE, Hunyuan, Doubao, and MiMo.

## Provider Summary

| Provider | Module | Auth env var | Context | Residency | Free tier |
|----------|--------|-------------|---------|-----------|-----------|
| DeepSeek | `deepseek` | `DEEPSEEK_API_KEY` | 64k | CN | No |
| Qwen (DashScope) | `qwen` | `DASHSCOPE_API_KEY` | up to 1M | Intl/CN | No |
| GLM (Zhipu AI) | `glm` | `GLM_API_KEY` | 128-256k | CN | No |
| Kimi (Moonshot AI) | `kimi` | `MOONSHOT_API_KEY` | up to 1M | Intl/CN | No |
| MiniMax | `minimax` | `MINIMAX_API_KEY` | up to 1M | CN | No |
| StepFun | `stepfun` | `STEPFUN_API_KEY` | up to 256k | CN | No |
| Baidu ERNIE | `ernie` | `ERNIE_API_KEY` | 8k | CN | No |
| Tencent Hunyuan | `hunyuan` | `HUNYUAN_API_KEY` | up to 256k | CN | hunyuan-lite |
| ByteDance Doubao | `doubao` | `DOUBAO_API_KEY` | up to 256k | CN | No |
| Xiaomi MiMo | `mimo` | `MIMO_API_KEY` (self-host) | 32k | Unknown | Yes (self-host) |

## Data Residency

All cloud-hosted Chinese providers route traffic through CN infrastructure.
If your workload has GDPR or similar constraints, use:

- `qwen-eu` -- Frankfurt (Alibaba EU endpoint)
- `kimi` -- Moonshot international endpoint (non-CN)
- `mimo` / `mimo-local` -- self-hosted; residency is yours to control

```rust
use ancora_inference::policy::{is_allowed, ResidencyTag};

let excluded = vec![ResidencyTag::Cn];
assert!(!is_allowed("doubao", &excluded));
assert!(is_allowed("qwen-eu", &excluded));
assert!(is_allowed("mimo", &excluded)); // self-hosted
```

## StepFun

Long-context specialist from Beijing-based Step AI. The 256k model is unique
among Chinese providers for pure text tasks at that length.

```rust
use ancora_inference::providers::stepfun::build_stepfun_profile;

let profile = build_stepfun_profile();
// Aliases: step-256k, step-128k, step-32k, step-v
```

Models:

| Model | Context | Input $/M | Output $/M | Tools | Vision |
|-------|---------|-----------|------------|-------|--------|
| step-1-256k | 256k | $0.45 | $0.45 | No | No |
| step-1-128k | 128k | $0.20 | $0.20 | Yes | No |
| step-1-32k | 32k | $0.07 | $0.07 | Yes | No |
| step-1v-8k | 8k | $0.10 | $0.10 | No | Yes |

## Baidu ERNIE

Baidu's Qianfan platform exposes ERNIE via an OpenAI-compatible endpoint.
The older OAuth flow (exchanging `client_id` + `client_secret` for an
access token) is no longer required; use an API key directly.

```rust
use ancora_inference::providers::ernie::build_ernie_profile;

let profile = build_ernie_profile();
// Aliases: ernie4, ernie3.5, speed, lite
```

Models:

| Model | Context | Input $/M | Output $/M | Tools |
|-------|---------|-----------|------------|-------|
| ernie-4.0-8k | 8k | $0.12 | $0.12 | Yes |
| ernie-3.5-8k | 8k | $0.05 | $0.05 | Yes |
| ernie-speed-8k | 8k | $0.004 | $0.008 | No |
| ernie-lite-8k | 8k | $0.003 | $0.006 | No |

## Tencent Hunyuan

Tencent's LLM platform. The `hunyuan-lite` tier is free and has a 256k
context window, making it useful for long-document processing at zero cost.

```rust
use ancora_inference::providers::hunyuan::build_hunyuan_profile;

let profile = build_hunyuan_profile();
// Aliases: turbo, pro, standard, vision, lite
```

Models:

| Model | Context | Input $/M | Output $/M | Tools | Vision |
|-------|---------|-----------|------------|-------|--------|
| hunyuan-turbo | 128k | $0.15 | $0.50 | Yes | No |
| hunyuan-pro | 32k | $0.45 | $1.50 | Yes | No |
| hunyuan-standard | 32k | $0.05 | $0.05 | No | No |
| hunyuan-vision | 8k | $0.18 | $0.18 | No | Yes |
| hunyuan-lite | 256k | $0.00 | $0.00 | No | No |

## ByteDance Doubao

ByteDance's Volcano Engine ARK platform. Doubao models are among the
cheapest production-grade Chinese options for 32k contexts.

```rust
use ancora_inference::providers::doubao::build_doubao_profile;

let profile = build_doubao_profile();
// Aliases: pro-32k, pro-256k, lite, vision, thinking, character
```

Models:

| Model | Context | Input $/M | Output $/M | Tools | Vision |
|-------|---------|-----------|------------|-------|--------|
| doubao-1.5-pro-32k | 32k | $0.04 | $0.08 | Yes | No |
| doubao-1.5-pro-256k | 256k | $0.11 | $0.22 | Yes | No |
| doubao-1.5-lite-32k | 32k | $0.01 | $0.03 | No | No |
| doubao-1.5-vision-32k | 32k | $0.08 | $0.08 | No | Yes |
| doubao-1.5-thinking-32k | 32k | $0.06 | $0.12 | No | No |
| doubao-character-128k | 128k | $0.05 | $0.10 | Yes | No |

For self-hosted Doubao-compatible endpoints:

```rust
use ancora_inference::providers::doubao::build_doubao_self_host_profile;

let profile = build_doubao_self_host_profile("http://your-server:8000/v1");
```

## Xiaomi MiMo

MiMo is Xiaomi's open-source reasoning model. There is no official cloud
endpoint; users deploy the weights themselves via vLLM, Ollama, or similar.

```rust
use ancora_inference::providers::mimo::build_mimo_profile;
use ancora_inference::providers::mimo::build_mimo_noauth_profile;

// With auth (e.g., vLLM with an API key)
let profile = build_mimo_profile("http://localhost:8000/v1");

// Without auth (purely local, no token required)
let profile = build_mimo_noauth_profile("http://localhost:8000/v1");
```

Models (both profiles include the same catalog):

| Model | Context | Tools |
|-------|---------|-------|
| mimo-7b-rl | 32k | No |
| mimo-7b | 32k | No |
| mimo-7b-rl-fc | 32k | Yes (FC adapter) |

Since MiMo runs locally, pricing is $0.00/M for all models.
