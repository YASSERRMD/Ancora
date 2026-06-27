# Cost and OTEL Tracing Example

Demonstrates wrapping an agent run in an in-process `Span` to record event
counts, estimated token usage, and duration -- matching what an OTEL exporter
would consume.

## What it tests

- `Span` records its name, attributes, and elapsed duration via `endMs()`
- `TokenEstimator.estimateTokens` returns at least 1 for any input
- Four characters map to one token (`Math.ceil(length / 4.0)`)
- Attributes set on a span are readable after `endMs()` is called
- A summary span can aggregate totals across multiple events

## Pattern

```java
try (Agent agent = new Agent()) {
    Span root = new Span("agent.run");

    List<RunEvent> events = agent.run(
        new AgentSpec("local-model", "Respond.", null, null, null)
    ).collectAll();

    long totalTokens = events.stream()
        .filter(e -> e instanceof RunEvent.Token)
        .mapToLong(e -> TokenEstimator.estimateTokens(((RunEvent.Token) e).text()))
        .sum();

    root.setAttribute("event.count", events.size());
    root.setAttribute("tokens.estimated", totalTokens);
    long durationMs = root.endMs();
    assertTrue(durationMs >= 0);
}
```

## Offline behaviour

`Span` and `TokenEstimator` tests run entirely in-process. The agent run
uses `Assumptions.assumeTrue(AncoraNative.AVAILABLE)`.
