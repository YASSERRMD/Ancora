# Durability and Restart Recovery (.NET)

## Enable durability

```csharp
using Ancora;

var store = new SqliteStore("/var/lib/myapp/journal.db");
var rt = new Runtime(new RuntimeOptions { Transport = new StoringTransport(store) });
await using var agent = new Agent(rt);
```

With a `StoringTransport`, every run is journalled automatically. If the
process restarts mid-run, replay the journal to continue:

```csharp
var handle = agent.Resume("run-abc-123");
await foreach (var ev in handle.Events())
{
    if (ev is CompletedEvent completed)
        Console.WriteLine(completed.Output);
}
```

## Deterministic run IDs

```csharp
var handle = agent.Run(spec, "Summarise the report.", new RunOptions
{
    RunId = "report-summary-2026-06-28"
});
```

Re-running with the same `RunId` replays completed activities and re-runs
only the remaining steps.

## In-memory store (tests)

```csharp
var rt = new Runtime(new RuntimeOptions { Transport = new StoringTransport(new MemoryStore()) });
```

## Idempotency key templates

```csharp
new ToolSpec
{
    Name = "send_email",
    Description = "Send an email.",
    IdempotencyKeyTemplate = "send_email/{runId}/{seq}",
    // ...
}
```

## See also

- [Observability](observability.md)
- [Durability concept](../../concepts/durability-and-replay.md)
