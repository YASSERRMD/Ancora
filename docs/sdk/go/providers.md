# Choosing a Provider

The `model_id` field in `AgentSpec` selects the model. The active provider
is determined by the environment variables you set.

## Local (default)

```bash
# No API key needed
ANCORA_MODEL_URL=http://127.0.0.1:11434
```

```go
spec := ancora.NewAgentSpec("llama3", "You are helpful.")
```

## Anthropic Claude

```bash
export ANTHROPIC_API_KEY=sk-ant-...
```

```go
spec := ancora.NewAgentSpec("claude-3-5-haiku-20241022", "You are helpful.")
```

## OpenAI

```bash
export OPENAI_API_KEY=sk-...
```

```go
spec := ancora.NewAgentSpec("gpt-4o-mini", "You are helpful.")
```

## Google Gemini

```bash
export GEMINI_API_KEY=...
```

```go
spec := ancora.NewAgentSpec("gemini-1.5-flash", "You are helpful.")
```

## Switching at runtime

Pass a `NewTransportAgent` to override the default provider for a single run:

```go
transport := ancora.NewCgoTransport(runtime)
agent := ancora.NewTransportAgent(transport)
```

## See also

- [Providers concept](../../concepts/providers-and-local-first.md)
- [Chinese providers](chinese-providers.md)
