# Streaming with IAsyncEnumerable (.NET)

Consume run events via `IAsyncEnumerable<RunEvent>` as they arrive.

## Basic streaming

```csharp
using Ancora;

var rt = new Runtime();
await using var agent = new Agent(rt);
var spec = new AgentSpec { Model = "llama3", Instructions = "Tell a short story." };

await foreach (var ev in agent.Run(spec, "Once upon a time...").Events())
{
    if (ev is TokenEvent token)
        Console.Write(token.Token);
}
Console.WriteLine();
```

## Accumulating tokens

```csharp
var tokens = new System.Text.StringBuilder();

await foreach (var ev in agent.Run(spec, prompt).Events())
{
    if (ev is TokenEvent token)
        tokens.Append(token.Token);
}

string fullText = tokens.ToString();
```

## Event types

| Type | Properties | Description |
|------|-----------|-------------|
| `StartedEvent` | `RunId` | Run has begun |
| `TokenEvent` | `Token` | One model output token |
| `ToolCallEvent` | `Name`, `Input` | Agent called a tool |
| `CompletedEvent` | `Output`, `Usage` | Run finished |
| `ResumedEvent` | `RunId` | Run resumed after pause |

## Streaming into a channel

```csharp
using System.Threading.Channels;

var channel = Channel.CreateUnbounded<string>();

_ = Task.Run(async () =>
{
    await foreach (var ev in agent.Run(spec, prompt).Events())
    {
        if (ev is TokenEvent token)
            await channel.Writer.WriteAsync(token.Token);
        else if (ev is CompletedEvent)
            channel.Writer.Complete();
    }
});

await foreach (var token in channel.Reader.ReadAllAsync())
    Console.Write(token);
```

## Cancellation

```csharp
using var cts = new CancellationTokenSource(TimeSpan.FromSeconds(30));

await foreach (var ev in agent.Run(spec, prompt).Events(cts.Token))
{
    if (ev is TokenEvent token)
        Console.Write(token.Token);
}
```

## See also

- [Human-in-the-loop](human-in-the-loop.md)
- [Quickstart](quickstart.md)
