# Single Agent Example

Demonstrates starting a run, collecting events, and verifying the event
sequence returned by the Ancora .NET SDK.

## What it tests

- `Agent.Run(AgentSpec)` returns a `RunHandle`
- `CollectAsync()` returns a non-empty event list
- The first event is `StartedEvent`
- The last event is `CompletedEvent`
- `RunHandle.RunId` is non-empty

## Pattern

```csharp
using var agent = new Agent();
var spec = new AgentSpec("local-model", "Respond with a greeting.");
var events = await agent.Run(spec).CollectAsync();
Assert.IsType<StartedEvent>(events[0]);
Assert.IsType<CompletedEvent>(events[^1]);
```

## Offline behaviour

When the native Ancora library is not present the test catches
`DllNotFoundException` and exits without failing CI.
