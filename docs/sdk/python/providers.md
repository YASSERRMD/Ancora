# Choosing a Provider (Python)

Ancora routes model requests through the provider configured by the
`ANCORA_MODEL_URL` environment variable. No code change is needed to switch
providers.

## Ollama (default, local)

```bash
export ANCORA_MODEL_URL="http://127.0.0.1:11434"
ollama pull llama3
```

```python
spec = AgentSpec(model="llama3", instructions="Answer.")
```

## Anthropic Claude

```bash
export ANCORA_MODEL_URL="https://api.anthropic.com/v1"
export ANTHROPIC_API_KEY="sk-ant-..."
```

```python
spec = AgentSpec(model="claude-3-5-haiku-20241022", instructions="Answer.")
```

## OpenAI

```bash
export ANCORA_MODEL_URL="https://api.openai.com/v1"
export OPENAI_API_KEY="sk-..."
```

```python
spec = AgentSpec(model="gpt-4o-mini", instructions="Answer.")
```

## Google Gemini

```bash
export ANCORA_MODEL_URL="https://generativelanguage.googleapis.com/v1beta"
export GEMINI_API_KEY="AIza..."
```

```python
spec = AgentSpec(model="gemini-2.0-flash", instructions="Answer.")
```

## Azure OpenAI

```bash
export ANCORA_MODEL_URL="https://<resource>.openai.azure.com/openai/deployments/<deployment>"
export AZURE_OPENAI_API_KEY="..."
export AZURE_OPENAI_API_VERSION="2024-08-01-preview"
```

## Runtime override per run

```python
spec = AgentSpec(
    model="llama3",
    instructions="Answer.",
    model_url="http://127.0.0.1:11434",   # overrides env var for this run only
)
```

## See also

- [Chinese providers](chinese-providers.md)
- [Install](install.md)
