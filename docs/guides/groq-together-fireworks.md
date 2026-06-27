# Groq, Together AI, and Fireworks AI

These three providers expose OpenAI-compatible REST APIs. The `OpenAiClient`
adapter works without modification -- only a different `ProviderProfile` is needed.

---

## Why use a throughput host?

| Goal | Good choice |
|---|---|
| Lowest latency for short outputs | Groq (LPU hardware) |
| Widest open-source model selection | Together AI |
| Highest sustained RPS / best cold-start | Fireworks AI |
| Cost: small model, very high volume | Any -- Llama 8B at $0.05-0.20/M tokens |

---

## Groq

### Overview

Groq runs on Language Processing Units (LPUs), dedicated silicon optimized
for autoregressive generation. It achieves very low time-to-first-token (TTFT)
and high tokens-per-second at the cost of a smaller model selection and stricter
rate limits on the free tier.

### Setup

```bash
export GROQ_API_KEY=gsk_...
```

### Base URL

`https://api.groq.com/openai` -- the `/openai` segment is part of the base URL,
so the standard `/v1/chat/completions` path appends correctly without changes to
the OpenAI client.

### Models

| Alias | Resolved ID | Context | Tools | Input $/M |
|---|---|---|---|---|
| `llama-3.3-70b` | `llama-3.3-70b-versatile` | 128k | Yes | $0.59 |
| `llama-3.1-8b` | `llama-3.1-8b-instant` | 128k | No | $0.05 |
| `llama3-70b` | `llama3-70b-8192` | 8k | Yes | $0.59 |
| `llama3-8b` | `llama3-8b-8192` | 8k | No | $0.05 |
| `mixtral` | `mixtral-8x7b-32768` | 32k | No | $0.24 |
| `gemma2` | `gemma2-9b-it` | 8k | No | $0.20 |

### Rate limits (free tier)

| Metric | Limit |
|---|---|
| Requests/minute | 30 |
| Tokens/minute | 6,000 |
| Retry-After header | Yes (honor it) |

---

## Together AI

### Overview

Together AI hosts dozens of open-source models across the Llama, Mistral,
Qwen, and DeepSeek families. It offers large context windows and competitive
pricing, particularly for the Llama 3.1 family at 128k tokens.

### Setup

```bash
export TOGETHER_API_KEY=...
```

### Base URL

`https://api.together.xyz`

### Models

| Alias | Resolved ID | Context | Tools | Input $/M |
|---|---|---|---|---|
| `llama3.1-70b` | `meta-llama/Meta-Llama-3.1-70B-Instruct-Turbo` | 128k | Yes | $0.88 |
| `llama3.1-8b` | `meta-llama/Meta-Llama-3.1-8B-Instruct-Turbo` | 128k | No | $0.18 |
| `llama3-70b` | `meta-llama/Llama-3-70b-chat-hf` | 8k | No | $0.90 |
| `mixtral` | `mistralai/Mixtral-8x7B-Instruct-v0.1` | 32k | No | $0.60 |
| `mistral-7b` | `mistralai/Mistral-7B-Instruct-v0.3` | 32k | No | $0.20 |
| `qwen2.5-72b` | `Qwen/Qwen2.5-72B-Instruct-Turbo` | 32k | Yes | $1.20 |
| `deepseek-r1` | `deepseek-ai/DeepSeek-R1` | 64k | No | $3.00 |

### Rate limits (free tier)

| Metric | Limit |
|---|---|
| Requests/minute | 60 |
| Tokens/minute | 200,000 |
| Retry-After header | No (use exponential backoff) |

---

## Fireworks AI

### Overview

Fireworks AI offers very high sustained throughput and competitive pricing for
the largest models in the Llama 3.1 family. Model IDs use the
`accounts/fireworks/models/<slug>` path format.

### Setup

```bash
export FIREWORKS_API_KEY=fw_...
```

### Base URL

`https://api.fireworks.ai/inference` -- the `/inference` segment is part of
the base URL.

### Models

| Alias | Resolved ID | Context | Tools | Input $/M |
|---|---|---|---|---|
| `llama3.1-70b` | `accounts/fireworks/models/llama-v3p1-70b-instruct` | 128k | Yes | $0.90 |
| `llama3.1-8b` | `accounts/fireworks/models/llama-v3p1-8b-instruct` | 128k | No | $0.20 |
| `llama3.1-405b` | `accounts/fireworks/models/llama-v3p1-405b-instruct` | 128k | Yes | $3.00 |
| `mixtral-8x22b` | `accounts/fireworks/models/mixtral-8x22b-instruct` | 64k | No | $1.20 |
| `mixtral` | `accounts/fireworks/models/mixtral-8x7b-instruct` | 32k | No | $0.50 |
| `qwen2.5-72b` | `accounts/fireworks/models/qwen2p5-72b-instruct` | 32k | Yes | $0.90 |

### Rate limits (free tier)

| Metric | Limit |
|---|---|
| Requests/minute | 600 |
| Tokens/minute | 1,000,000 |
| Retry-After header | No (use exponential backoff) |

---

## Summary comparison

| Feature | Groq | Together | Fireworks |
|---|---|---|---|
| Wire format | OpenAI-compatible | OpenAI-compatible | OpenAI-compatible |
| Auth env var | `GROQ_API_KEY` | `TOGETHER_API_KEY` | `FIREWORKS_API_KEY` |
| Base URL suffix | `/openai` | (none) | `/inference` |
| Speciality | Ultra-low latency | Model variety | High RPS / 405B |
| Retry-After on 429 | Yes | No | No |
| Free tier RPM | 30 | 60 | 600 |

## Using the health probe

```rust
use ancora_inference::providers::throughput::{health_probe, HealthStatus};
use ancora_inference::providers::groq::build_groq_profile;

let profile = build_groq_profile();
match health_probe(&profile) {
    HealthStatus::Ready => println!("groq is ready"),
    HealthStatus::MissingCredential(var) => eprintln!("set {var}"),
    HealthStatus::InvalidProfile(msg) => eprintln!("bad profile: {msg}"),
}
```
