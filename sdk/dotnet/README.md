# Ancora .NET SDK

Local-first agentic framework bindings for .NET 8+.

## Prerequisites

- .NET 8 SDK or later
- The `ancora_ffi` native library (`.dylib` on macOS, `.so` on Linux, `.dll` on Windows)

### Building the native library

```bash
# from the repo root
cargo build -p ancora-ffi --release
# macOS: target/release/libancora_ffi.dylib
# Linux: target/release/libancora_ffi.so
```

Place the library where the .NET runtime can find it:
- Next to the test binary (for local testing)
- In a directory listed in `LD_LIBRARY_PATH` (Linux) or `DYLD_LIBRARY_PATH` (macOS)
- Bundled in the NuGet package under `runtimes/<rid>/native/`

## Installation

```xml
<!-- add to your .csproj -->
<PackageReference Include="Ancora.Agent" Version="0.1.0" />
```

```bash
dotnet add package Ancora.Agent
```

The package id is `Ancora.Agent` (the plain `Ancora` id on nuget.org belongs
to an unrelated library), but the assembly and namespace are still `Ancora`,
so your code writes `using Ancora;` regardless.

## Quickstart: run a single agent

```csharp
using Ancora;

// Create a runtime and an agent.
using var agent = new Agent();

// Describe the agent.
var spec = new AgentSpec(
    Model: "llama3",
    Instructions: "You are a concise assistant. Answer in one sentence."
);

// Start the run and stream events.
var handle = agent.Run(spec);

await foreach (var ev in handle.EventsAsync())
{
    switch (ev)
    {
        case StartedEvent s:
            Console.WriteLine($"Run started: {s.RunId}");
            break;
        case TokenEvent t:
            Console.Write(t.Text);
            break;
        case CompletedEvent:
            Console.WriteLine("\nDone.");
            break;
    }
}
```

## Register a tool

```csharp
using Ancora;
using System.Text.Json;

using var rt = new Runtime();
using var agent = new Agent(rt);

// Register a tool by delegate.
using var reg = ToolRegistry.Register(rt, "get_time", "Return the current UTC time",
    input => """{"time":"2025-01-01T00:00:00Z"}""");

var spec = new AgentSpec("llama3", "Tell me the time.", Tools: [
    new ToolSpec("get_time", "Return the current UTC time")
]);

var handle = agent.Run(spec);
var events = await handle.CollectAsync();
```

## Attribute-based tools

```csharp
using Ancora;

public class MyTools
{
    [Tool("Echo a message back")]
    public string Echo([ToolInput("The text to echo")] string text)
        => $"{{\"echo\":\"{text}\"}}";
}

using var rt = new Runtime();
var tools = new MyTools();
var registrations = ToolRegistry.RegisterAll(rt, tools);
// each registration is IDisposable; dispose to unregister
```

## Human-in-the-loop

```csharp
var handle = agent.Run(spec);

// Drain initial events.
var initial = await handle.CollectAsync();

// Inject a human decision and continue.
var afterDecision = await handle.ResumeAndCollectAsync("approved");
```

## Cost summary

```csharp
var handle = agent.Run(spec);
await handle.CollectAsync();
Console.WriteLine(handle.GetCost());
// {"run_id":"...","total_usd":0}
```

## Project layout

```
sdk/dotnet/
  Ancora/
    Interop/        -- raw DllImport declarations (ancora-ffi C ABI)
    Handles/        -- SafeHandle wrappers for opaque FFI pointers
    AgentSpec.cs    -- AgentSpec, ToolSpec, GraphSpec records
    RunEvent.cs     -- RunEvent discriminated union
    Wire.cs         -- JSON serialization helpers
    Agent.cs        -- Agent class (start runs)
    RunHandle.cs    -- IAsyncEnumerable event stream + resume
    ToolRegistry.cs -- attribute-based and delegate-based tool registration
  Ancora.Tests/     -- xUnit test suite
```

## License

Apache-2.0
