# Concurrency (.NET)

## Concurrent runs with `Task.WhenAll`

`Agent` is safe for concurrent use. Multiple `agent.Run()` calls can overlap:

```csharp
using Ancora;

var rt = new Runtime();
await using var agent = new Agent(rt);
var spec = new AgentSpec { Model = "llama3", Instructions = "Summarise." };

var prompts = new[] { "Text A", "Text B", "Text C", "Text D" };

async Task<string> RunOne(string prompt)
{
    string output = "";
    await foreach (var ev in agent.Run(spec, prompt).Events())
        if (ev is CompletedEvent c) output = c.Output;
    return output;
}

var results = await Task.WhenAll(prompts.Select(RunOne));

foreach (var r in results)
    Console.WriteLine(r[..Math.Min(60, r.Length)]);
```

## Limiting concurrency with `SemaphoreSlim`

```csharp
var semaphore = new SemaphoreSlim(4);

async Task<string> RunThrottled(string prompt)
{
    await semaphore.WaitAsync();
    try { return await RunOne(prompt); }
    finally { semaphore.Release(); }
}

var results = await Task.WhenAll(prompts.Select(RunThrottled));
```

## Streaming into `System.Threading.Channels`

```csharp
var channel = Channel.CreateUnbounded<string>();

_ = Task.Run(async () =>
{
    await foreach (var ev in agent.Run(spec, prompt).Events())
    {
        if (ev is TokenEvent t) await channel.Writer.WriteAsync(t.Token);
        else if (ev is CompletedEvent) channel.Writer.Complete();
    }
});

await foreach (var token in channel.Reader.ReadAllAsync())
    Console.Write(token);
```

## See also

- [Streaming](streaming.md)
- [Multi-agent graphs](multi-agent.md)
