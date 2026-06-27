# Quickstart (.NET)

Run a minimal agent locally in under five minutes.

## Prerequisites

- `dotnet add package Ancora`
- Ollama running: `ollama serve && ollama pull llama3`

## Minimal example

```csharp
using Ancora;

var rt = new Runtime();
await using var agent = new Agent(rt);

var spec = new AgentSpec
{
    Model = "llama3",
    Instructions = "Answer concisely.",
};

await foreach (var ev in agent.Run(spec, "What is a durable agent?").Events())
{
    if (ev is CompletedEvent completed)
        Console.WriteLine(completed.Output);
}
```

## Run it

```bash
dotnet run
```

## What happened

1. `new Runtime()` initialises the native Ancora engine via P/Invoke.
2. `new Agent(rt)` creates an agent bound to the runtime.
3. `agent.Run(spec, prompt).Events()` returns an `IAsyncEnumerable<RunEvent>`.
4. `CompletedEvent.Output` holds the final model response.

## Skipping when native library is absent

```csharp
// In xUnit tests:
try { new Runtime(); }
catch (DllNotFoundException) { return; }
```

## Next steps

- [Defining tools](tools.md) -- register C# delegates as agent tools
- [Structured output](structured-output.md) -- deserialise output with System.Text.Json
- [Streaming](streaming.md) -- process events as they arrive
