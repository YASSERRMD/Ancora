# .NET SDK API Reference

## `Runtime`

```csharp
public sealed class Runtime : IDisposable
{
    public Runtime() { }
    public Runtime(RuntimeOptions options) { }
    public void Dispose() { }
}
```

## `Agent`

```csharp
public sealed class Agent : IDisposable, IAsyncDisposable
{
    public Agent(Runtime runtime) { }

    public RunHandle Run(AgentSpec spec, string prompt = "", RunOptions? options = null);
    public RunHandle Start(AgentSpec spec, string prompt = "", RunOptions? options = null);
    public RunHandle Resume(string runId);
    public GraphHandle RunGraph(GraphSpec graph, string prompt = "");
    public void Dispose() { }
    public ValueTask DisposeAsync() { }
}
```

## `AgentSpec`

```csharp
public sealed class AgentSpec
{
    public string Model { get; set; } = "";
    public string Instructions { get; set; } = "";
    public List<ToolSpec> Tools { get; set; } = new();
    public int MaxTokens { get; set; } = 4096;
    public float Temperature { get; set; } = 0.7f;
    public Dictionary<string, object>? OutputSchema { get; set; }
    public PolicySpec? Policy { get; set; }
    public List<string> McpServers { get; set; } = new();
    public string? ModelUrl { get; set; }
}
```

## `ToolSpec`

```csharp
public sealed class ToolSpec
{
    public string Name { get; set; } = "";
    public string Description { get; set; } = "";
    public ToolInputSchema InputSchema { get; set; } = new();
    public EffectClass Effect { get; set; } = EffectClass.None;
    public string? IdempotencyKeyTemplate { get; set; }
    public Func<JsonElement, string>? Fn { get; set; }
    public Func<JsonElement, Task<string>>? AsyncFn { get; set; }
}
```

## `RunEvent` hierarchy

```csharp
public abstract class RunEvent { }
public sealed class StartedEvent : RunEvent { public string RunId { get; } }
public sealed class TokenEvent : RunEvent { public string Token { get; } }
public sealed class ToolCallEvent : RunEvent { public string Name { get; } public JsonElement Input { get; } }
public sealed class CompletedEvent : RunEvent { public string Output { get; } public TokenUsage Usage { get; } }
public sealed class ResumedEvent : RunEvent { public string RunId { get; } }
```

## `RunHandle`

```csharp
public sealed class RunHandle
{
    public string RunId { get; }
    public RunStatus Status { get; }
    public string? PauseReason { get; }

    public IAsyncEnumerable<RunEvent> Events(CancellationToken ct = default);
    public Task RunUntilPauseAsync(CancellationToken ct = default);
    public Task ResumeAsync(string payload);
    public Task ResumeBytesAsync(byte[] payload);
}
```

## `PolicySpec`

```csharp
public sealed class PolicySpec
{
    public List<string> AllowRegions { get; set; } = new();
    public List<string> DenyProviders { get; set; } = new();
    public int MaxWriteTools { get; set; }
}
```

## `SqliteStore` / `MemoryStore`

```csharp
public sealed class SqliteStore { public SqliteStore(string path) { } }
public sealed class MemoryStore { public bool HasRun(string runId) { } }
```

## `GraphSpec`

```csharp
public sealed class GraphSpec
{
    public List<GraphNode> Nodes { get; set; } = new();
    public List<GraphEdge> Edges { get; set; } = new();
}

public sealed class GraphNode { public string Id { get; set; } = ""; public AgentSpec Spec { get; set; } = new(); }
public sealed class GraphEdge { public string From { get; set; } = ""; public string To { get; set; } = ""; }
```
