# Streaming Chat Example

Demonstrates consuming agent events via the `Iterable<RunEvent>` loop,
concatenating token text as it arrives.

## What it tests

- `handle.events()` is iterable with a for-each loop
- Token text from `RunEvent.Token.text()` can be concatenated via a
  `StringBuilder`
- The stream ends with a `RunEvent.Completed` event
- `collectAll()` and the `events()` iterable both return the full event list

## Pattern

```java
try (Agent agent = new Agent()) {
    AgentSpec spec = new AgentSpec("local-model", "Stream a short reply.", null, null, null);
    var handle = agent.run(spec);

    var sb = new StringBuilder();
    RunEvent last = null;

    for (RunEvent ev : handle.events()) {
        if (ev instanceof RunEvent.Token tok)
            sb.append(tok.text());
        last = ev;
    }

    assertInstanceOf(RunEvent.Completed.class, last);
}
```

## Offline behaviour

`Assumptions.assumeTrue(AncoraNative.AVAILABLE)` and `UnsatisfiedLinkError`
catch guard all FFI-dependent paths.
