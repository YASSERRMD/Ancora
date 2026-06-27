# Choosing a Provider (.NET)

Set `ANCORA_MODEL_URL` to switch inference providers. No code change required.

## Ollama (default, local)

```bash
export ANCORA_MODEL_URL="http://127.0.0.1:11434"
ollama pull llama3
```

```csharp
var spec = new AgentSpec { Model = "llama3", Instructions = "Answer." };
```

## Anthropic Claude

```bash
export ANCORA_MODEL_URL="https://api.anthropic.com/v1"
export ANTHROPIC_API_KEY="sk-ant-..."
```

```csharp
var spec = new AgentSpec { Model = "claude-3-5-haiku-20241022", Instructions = "Answer." };
```

## OpenAI

```bash
export ANCORA_MODEL_URL="https://api.openai.com/v1"
export OPENAI_API_KEY="sk-..."
```

```csharp
var spec = new AgentSpec { Model = "gpt-4o-mini", Instructions = "Answer." };
```

## Azure OpenAI

```bash
export ANCORA_MODEL_URL="https://<resource>.openai.azure.com/openai/deployments/<deployment>"
export AZURE_OPENAI_API_KEY="..."
```

## Per-spec model URL override

```csharp
var spec = new AgentSpec
{
    Model = "llama3",
    Instructions = "Answer.",
    ModelUrl = "http://127.0.0.1:11434",   // overrides env var
};
```

## Reading from appsettings.json

```csharp
// appsettings.json
{
  "Ancora": {
    "ModelUrl": "http://127.0.0.1:11434",
    "Model": "llama3"
  }
}

// Program.cs
var config = builder.Configuration.GetSection("Ancora");
var spec = new AgentSpec
{
    Model = config["Model"]!,
    ModelUrl = config["ModelUrl"],
    Instructions = "Answer.",
};
```

## See also

- [GLM self-host](glm-selfhost.md)
- [Install](install.md)
