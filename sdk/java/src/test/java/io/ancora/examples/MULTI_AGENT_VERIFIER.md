# Multi-Agent Verifier Example

Demonstrates running a primary agent and a verifier agent concurrently using
`CompletableFuture.supplyAsync`, sharing a single `Agent` instance.

## What it tests

- Two `Agent.run` calls return distinct run IDs
- Both `CompletableFuture` tasks complete without error
- Results from both are non-empty

## Pattern

```java
try (Agent agent = new Agent()) {
    var h1 = agent.run(new AgentSpec("local-model", "Produce an answer.", null, null, null));
    var h2 = agent.run(new AgentSpec("local-model", "Verify the answer.", null, null, null));

    assertNotEquals(h1.runId(), h2.runId());

    var f1 = CompletableFuture.supplyAsync(() -> { try { return h1.collectAll(); } catch (Throwable t) { throw new RuntimeException(t); } });
    var f2 = CompletableFuture.supplyAsync(() -> { try { return h2.collectAll(); } catch (Throwable t) { throw new RuntimeException(t); } });

    assertFalse(f1.join().isEmpty());
    assertFalse(f2.join().isEmpty());
}
```

## Offline behaviour

`Assumptions.assumeTrue(AncoraNative.AVAILABLE)` and `UnsatisfiedLinkError`
catch guard all FFI-dependent paths.
