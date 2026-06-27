# Cost and OTEL Tracing Example

Demonstrates wrapping an agent run in an in-process `Span` to record event
counts, estimated token usage, and duration -- matching what an OTEL exporter
would consume.

## What it tests

- `Span` records its name, attributes, and elapsed duration via `EndMs()`
- `TokenEstimator.EstimateTokens` returns at least 1 for any input
- Four characters map to one token (`Math.Ceiling(length / 4.0)`)
- Attributes set on a span are readable after `EndMs()` is called
- A summary span can aggregate totals across multiple child spans

## Pattern

```csharp
using var agent = new Agent();
var root = new Span("agent.run");

var events = await agent.Run(new AgentSpec("local-model", "Respond.")).CollectAsync();
var totalTokens = events.OfType<TokenEvent>()
    .Sum(e => TokenEstimator.EstimateTokens(e.Text));

root.SetAttribute("event.count",      events.Count);
root.SetAttribute("tokens.estimated", totalTokens);
var durationMs = root.EndMs();

Assert.True(durationMs >= 0);
```

## Offline behaviour

`Span` and `TokenEstimator` tests run entirely in-process. The agent run
catches `DllNotFoundException`.
