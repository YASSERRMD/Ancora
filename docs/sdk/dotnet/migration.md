# Migration from Microsoft Agent Framework (.NET)

## From Microsoft.Extensions.AI

| Microsoft.Extensions.AI concept | Ancora equivalent |
|----------------------------------|-------------------|
| `IChatClient` | `Agent` |
| `ChatMessage` | `AgentSpec.Instructions` string |
| `AIFunction` | `ToolSpec` registered in `ToolRegistry` |
| `ChatOptions` | `AgentSpec.MaxTokens`, `AgentSpec.Temperature` |
| `StreamingChatCompletionUpdate` | `TokenEvent` in `IAsyncEnumerable<RunEvent>` |

### Before (Microsoft.Extensions.AI)

```csharp
using Microsoft.Extensions.AI;

IChatClient client = new OllamaChatClient("http://localhost:11434", "llama3");

var response = await client.CompleteAsync(
    new[] { new ChatMessage(ChatRole.User, "What is a durable agent?") }
);

Console.WriteLine(response.Message.Text);
```

### After (Ancora)

```csharp
using Ancora;

var rt = new Runtime();
await using var agent = new Agent(rt);

var spec = new AgentSpec { Model = "llama3", Instructions = "Answer." };

await foreach (var ev in agent.Run(spec, "What is a durable agent?").Events())
{
    if (ev is CompletedEvent c) Console.WriteLine(c.Output);
}
```

## From Semantic Kernel

| Semantic Kernel concept | Ancora equivalent |
|------------------------|-------------------|
| `Kernel` | `Runtime` |
| `KernelFunction` | `ToolSpec` |
| `KernelPlugin` | `ToolRegistry` |
| `ChatHistory` | `AgentSpec.Instructions` with context |
| `IChatCompletionService` | `ANCORA_MODEL_URL` env var |
| `IMemoryStore` | `SqliteStore` + `StoringTransport` |

### Before (Semantic Kernel)

```csharp
using Microsoft.SemanticKernel;

var kernel = Kernel.CreateBuilder()
    .AddOllamaChatCompletion("llama3", new Uri("http://localhost:11434"))
    .Build();

var result = await kernel.InvokePromptAsync("What is a durable agent?");
Console.WriteLine(result);
```

### After (Ancora)

```csharp
var spec = new AgentSpec { Model = "llama3", Instructions = "Answer." };

await foreach (var ev in agent.Run(spec, "What is a durable agent?").Events())
{
    if (ev is CompletedEvent c) Console.WriteLine(c.Output);
}
```

## Key differences

- Ancora uses `IAsyncEnumerable<RunEvent>` for streaming (not callbacks).
- Durability is built-in via `StoringTransport` (no separate persistence plugin).
- Policy enforcement happens at the engine level (not middleware).
- Graphs replace orchestration plugins.

## See also

- [Multi-agent graphs](multi-agent.md)
- [Durability](durability.md)
