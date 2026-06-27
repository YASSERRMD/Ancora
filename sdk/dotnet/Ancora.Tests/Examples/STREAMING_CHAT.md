# Streaming Chat Example

Demonstrates consuming agent events via `IAsyncEnumerable<RunEvent>` using
the `await foreach` pattern, concatenating token text as it arrives.

## What it tests

- `EventsAsync()` is iterable with `await foreach`
- Token text from `TokenEvent.Text` can be concatenated
- The stream ends with a `CompletedEvent`

## Pattern

```csharp
using var agent = new Agent();
var handle = agent.Run(new AgentSpec("local-model", "Stream a short reply."));

var sb = new StringBuilder();
RunEvent? last = null;

await foreach (var ev in handle.EventsAsync())
{
    if (ev is TokenEvent tok)
        sb.Append(tok.Text);
    last = ev;
}

Assert.IsType<CompletedEvent>(last);
```

## Offline behaviour

`DllNotFoundException` is caught; the `await foreach` loop never executes
and the test exits cleanly.
