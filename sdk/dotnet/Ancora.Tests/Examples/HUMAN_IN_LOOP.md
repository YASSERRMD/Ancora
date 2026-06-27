# Human-in-Loop Example

Demonstrates pausing an agent run and resuming it with a human decision,
using both string and byte-array overloads of `RunHandle.Resume`.

## What it tests

- `CollectAsync()` returns events before `Resume` is called
- `Resume(string)` does not throw
- `Resume(byte[])` does not throw
- Post-resume events are accessible via a second `CollectAsync()` call

## Pattern

```csharp
using var agent = new Agent();
var handle = agent.Run(new AgentSpec("local-model", "Await decision."));

// collect pre-resume events
var preEvents = await handle.CollectAsync();
Assert.NotEmpty(preEvents);

// supply human decision as string
handle.Resume("approved");

// or as bytes
var bytes = Encoding.UTF8.GetBytes("approved");
handle.Resume(bytes);

// collect post-resume events
var postEvents = await handle.CollectAsync();
var hasResumed = postEvents.Any(e => e is ResumedEvent);
```

## Offline behaviour

`DllNotFoundException` is caught at the top of each test method.
