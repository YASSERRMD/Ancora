# Human-in-the-Loop (.NET)

Suspend a run at a tool call boundary and resume it with human input.

## Pattern

```csharp
using Ancora;

var rt = new Runtime();
await using var agent = new Agent(rt);

var registry = new ToolRegistry();
registry.Register(new ToolSpec
{
    Name = "request_approval",
    Description = "Ask a human to approve an action.",
    InputSchema = new ToolInputSchema
    {
        Type = "object",
        Properties = new Dictionary<string, ToolInputProperty>
        {
            ["action"] = new ToolInputProperty { Type = "string" }
        },
        Required = new List<string> { "action" }
    },
    Fn = args => throw new SuspendSignal($"Approve: {args["action"]!.GetString()}")
});

var spec = new AgentSpec
{
    Model = "llama3",
    Instructions = "Before modifying any file, call request_approval.",
    Tools = registry.ToList(),
};

var handle = agent.Start(spec, "Delete the temp directory.");
await handle.RunUntilPauseAsync();

if (handle.Status == RunStatus.Paused)
{
    Console.WriteLine("Approval required: " + handle.PauseReason);
    Console.Write("Type YES to approve: ");
    var answer = Console.ReadLine()!;
    await handle.ResumeAsync(answer);
}

await foreach (var ev in handle.Events())
{
    if (ev is CompletedEvent completed)
        Console.WriteLine(completed.Output);
}
```

## Resume with binary payload

```csharp
await handle.ResumeBytesAsync(
    System.Text.Json.JsonSerializer.SerializeToUtf8Bytes(new { approved = true })
);
```

## Timeout

```csharp
using var cts = new CancellationTokenSource(TimeSpan.FromSeconds(30));
cts.Token.Register(() => handle.Resume("TIMEOUT"));
```

## See also

- [Streaming](streaming.md)
- [Durability](durability.md)
