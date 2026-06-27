# Providers and Local-First

Ancora's inference layer is **provider-agnostic**. You can point the same
`AgentSpec` at a local Ollama instance or a hosted Anthropic endpoint by
changing one configuration value.

## Supported providers

| Provider | Type | Notes |
|----------|------|-------|
| Ollama | Local | Default; zero setup for development |
| llama.cpp / GGUF | Local | Edge and air-gapped deployments |
| Anthropic Claude | Remote | Opt-in; data leaves your network |
| OpenAI / Azure OpenAI | Remote | Opt-in |
| Google Gemini | Remote | Opt-in |
| DeepSeek | Remote | Opt-in; China-hosted option |
| Qwen (Alibaba) | Remote | Opt-in; China region |
| GLM (Zhipu AI) | Remote / Local | Also self-hostable via GGUF |
| Groq / Together / Fireworks | Remote | Opt-in |

## Provider configuration

Providers are configured via environment variables:

```
ANCORA_MODEL_URL=http://127.0.0.1:11434  # Ollama
ANTHROPIC_API_KEY=sk-...                 # Anthropic
OPENAI_API_KEY=sk-...                    # OpenAI
```

The `model_id` field in `AgentSpec` selects the specific model within the
active provider.

## Local-first principle

The default provider is Ollama with `llama3`. No API key is required. This
lets you:

- Build and test agents without an internet connection.
- Avoid sending sensitive data to remote services.
- Control costs (no per-token billing for local inference).

## See also

- [Providers guide](../guides/adding-a-provider.md)
- [Chinese providers guide](../guides/chinese-labs.md)
