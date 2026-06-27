# Choosing a Provider (Java)

Set `ANCORA_MODEL_URL` to switch inference providers. No code change required.

## Ollama (default, local)

```bash
export ANCORA_MODEL_URL="http://127.0.0.1:11434"
ollama pull llama3
```

```java
var spec = new AgentSpec("llama3", "Answer.", List.of(), 4096, 0.7f);
```

## Anthropic Claude

```bash
export ANCORA_MODEL_URL="https://api.anthropic.com/v1"
export ANTHROPIC_API_KEY="sk-ant-..."
```

```java
var spec = new AgentSpec("claude-3-5-haiku-20241022", "Answer.", List.of(), 4096, 0.7f);
```

## OpenAI

```bash
export ANCORA_MODEL_URL="https://api.openai.com/v1"
export OPENAI_API_KEY="sk-..."
```

```java
var spec = new AgentSpec("gpt-4o-mini", "Answer.", List.of(), 4096, 0.7f);
```

## Azure OpenAI

```bash
export ANCORA_MODEL_URL="https://<resource>.openai.azure.com/openai/deployments/<deployment>"
export AZURE_OPENAI_API_KEY="..."
```

## Per-spec model URL override

```java
var spec = new AgentSpec.Builder()
    .model("llama3")
    .instructions("Answer.")
    .modelUrl("http://127.0.0.1:11434")   // overrides env var
    .build();
```

## Reading from system properties

```java
String modelUrl = System.getProperty("ancora.modelUrl",
    System.getenv().getOrDefault("ANCORA_MODEL_URL", "http://127.0.0.1:11434"));
```

## See also

- [Qwen regional endpoints](qwen-regional.md)
- [Install](install.md)
