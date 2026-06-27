# Single Agent Example

Demonstrates starting a run, iterating events, and verifying the event
sequence returned by the Ancora Java SDK.

## What it tests

- `Agent.run(AgentSpec)` returns a `RunHandle`
- `collectAll()` returns a non-empty list
- First event is `RunEvent.Started`
- Last event is `RunEvent.Completed`
- `RunHandle.runId()` is non-blank

## Pattern

```java
try (Agent agent = new Agent()) {
    AgentSpec spec = new AgentSpec("local-model", "Respond with a greeting.", null, null, null);
    List<RunEvent> events = agent.run(spec).collectAll();
    assertInstanceOf(RunEvent.Started.class, events.get(0));
    assertInstanceOf(RunEvent.Completed.class, events.get(events.size() - 1));
}
```

## Offline behaviour

Tests call `Assumptions.assumeTrue(AncoraNative.AVAILABLE, ...)` and catch
`UnsatisfiedLinkError` so they skip cleanly when the native library is absent.
