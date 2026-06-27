# Choosing a Provider (TypeScript)

Set `ANCORA_MODEL_URL` to switch inference providers. No code change required.

## Ollama (default, local)

```bash
export ANCORA_MODEL_URL="http://127.0.0.1:11434"
ollama pull llama3
```

```ts
const spec = buildSpec({ model: 'llama3', instructions: 'Answer.' })
```

## Anthropic Claude

```bash
export ANCORA_MODEL_URL="https://api.anthropic.com/v1"
export ANTHROPIC_API_KEY="sk-ant-..."
```

```ts
const spec = buildSpec({ model: 'claude-3-5-haiku-20241022', instructions: 'Answer.' })
```

## OpenAI

```bash
export ANCORA_MODEL_URL="https://api.openai.com/v1"
export OPENAI_API_KEY="sk-..."
```

```ts
const spec = buildSpec({ model: 'gpt-4o-mini', instructions: 'Answer.' })
```

## Google Gemini

```bash
export ANCORA_MODEL_URL="https://generativelanguage.googleapis.com/v1beta"
export GEMINI_API_KEY="AIza..."
```

```ts
const spec = buildSpec({ model: 'gemini-2.0-flash', instructions: 'Answer.' })
```

## Azure OpenAI

```bash
export ANCORA_MODEL_URL="https://<resource>.openai.azure.com/openai/deployments/<deployment>"
export AZURE_OPENAI_API_KEY="..."
```

## Runtime override per run

```ts
const spec = buildSpec({
  model: 'llama3',
  instructions: 'Answer.',
  modelUrl: 'http://127.0.0.1:11434',   // overrides env var for this spec only
})
```

## See also

- [Chinese providers](chinese-providers.md)
- [Install](install.md)
