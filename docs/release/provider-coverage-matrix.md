# Provider Coverage Matrix

## Ancora 0.6.0

All providers run from recorded fixtures. No live API calls in CI.

| Provider | Region | Cost Model | Residency Tag | Languages |
|----------|--------|------------|---------------|-----------|
| Anthropic Claude | global | 3.0/15.0 USD per M | none | all 6 |
| OpenAI | global | configurable | none | all 6 |
| Qwen (Alibaba) | cn-hangzhou | configurable | cn | rust, go, python, ts, java |
| GLM (Zhipu) | cn-beijing | configurable | cn | dotnet |
| DeepSeek | cn | configurable | cn | rust, go, python |
| Kimi (Moonshot) | cn | configurable | cn | rust, go |
| MiniMax | cn | configurable | cn | rust |
| StepFun | cn | configurable | cn | rust |
| ERNIE (Baidu) | cn | configurable | cn | rust |
| Hunyuan (Tencent) | cn | configurable | cn | rust |
| Doubao (ByteDance) | cn | configurable | cn | rust |
| MiMo (Xiaomi) | cn | configurable | cn | rust |
| Ollama (local) | local | free | local | all 6 |
| LMStudio (local) | local | free | local | all 6 |

## Cost formula

```
cost_usd = (input_tokens / 1_000_000) * rate_in + (output_tokens / 1_000_000) * rate_out
```

Default Anthropic rates: `rate_in = 3.0`, `rate_out = 15.0`.

## Residency enforcement

- `cn` providers reject calls unless `data_residency = "cn"` is set.
- `local` providers always set `local_only = true`.
- `none` (global) providers accept any residency setting.
