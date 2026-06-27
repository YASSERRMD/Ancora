# Configuration (.NET)

## Environment variables

| Variable | Default | Description |
|----------|---------|-------------|
| `ANCORA_MODEL_URL` | `http://127.0.0.1:11434` | Inference endpoint URL |
| `ANCORA_LOG_LEVEL` | `warn` | Log level: `trace`, `debug`, `info`, `warn`, `error` |
| `ANTHROPIC_API_KEY` | (none) | API key for Anthropic endpoints |
| `OPENAI_API_KEY` | (none) | API key for OpenAI endpoints |
| `GEMINI_API_KEY` | (none) | API key for Google Gemini |
| `GLM_API_KEY` | (none) | API key for Zhipu GLM |
| `DASHSCOPE_API_KEY` | (none) | API key for Alibaba Qwen |
| `DEEPSEEK_API_KEY` | (none) | API key for DeepSeek |
| `AZURE_OPENAI_API_KEY` | (none) | API key for Azure OpenAI |

## `RuntimeOptions`

```csharp
var rt = new Runtime(new RuntimeOptions
{
    Transport = new StoringTransport(new SqliteStore("/var/lib/myapp/journal.db")),
    HttpTimeout = TimeSpan.FromMinutes(10),
    LogLevel = "info",
});
```

## Reading from `appsettings.json`

```json
{
  "Ancora": {
    "ModelUrl": "http://127.0.0.1:11434",
    "Model": "llama3",
    "JournalPath": "/var/lib/myapp/journal.db"
  }
}
```

```csharp
var section = builder.Configuration.GetSection("Ancora");
var rt = new Runtime(new RuntimeOptions
{
    Transport = new StoringTransport(new SqliteStore(section["JournalPath"]!))
});
var spec = new AgentSpec { Model = section["Model"]!, ModelUrl = section["ModelUrl"] };
```

## Dependency injection

```csharp
builder.Services.AddSingleton(new Runtime(new RuntimeOptions
{
    Transport = new StoringTransport(new SqliteStore("/var/lib/myapp/journal.db"))
}));
builder.Services.AddScoped<Agent>(sp => new Agent(sp.GetRequiredService<Runtime>()));
```

## See also

- [Install](install.md)
- [Providers](providers.md)
