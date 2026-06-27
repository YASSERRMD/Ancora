# Human-in-Loop Example

Demonstrates pausing an agent run and resuming it with a human decision,
using both `String` and `byte[]` overloads of `RunHandle.resume`.

## What it tests

- `collectAll()` returns events before `resume` is called
- `resume(String)` does not throw
- `resume(byte[])` does not throw
- Post-resume events are accessible via a second `collectAll()`

## Pattern

```java
try (Agent agent = new Agent()) {
    var handle = agent.run(new AgentSpec("local-model", "Await decision.", null, null, null));

    List<RunEvent> preEvents = handle.collectAll();
    assertFalse(preEvents.isEmpty());

    handle.resume("approved");

    byte[] bytes = "approved".getBytes(StandardCharsets.UTF_8);
    handle.resume(bytes);

    List<RunEvent> postEvents = handle.collectAll();
    boolean hasResumed = postEvents.stream().anyMatch(e -> e instanceof RunEvent.Resumed);
}
```

## Offline behaviour

`Assumptions.assumeTrue(AncoraNative.AVAILABLE)` and `UnsatisfiedLinkError`
guard all FFI paths.
