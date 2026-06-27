# Qwen Regional Gateway Example

Demonstrates iterating over multiple Qwen model variants using the same
`Agent` instance and verifying that each run receives a distinct `RunId`.

## What it tests

- All four Qwen model names are distinct strings starting with `qwen-`
- An `AgentSpec` can be built for each Qwen model variant
- Each `RunHandle.runId()` is unique across all variants

## Pattern

```java
List<String> qwenModels = List.of("qwen-turbo", "qwen-plus", "qwen-max", "qwen-long");

try (Agent agent = new Agent()) {
    List<String> runIds = new ArrayList<>();
    for (String model : qwenModels) {
        AgentSpec spec = new AgentSpec(model, "Respond briefly.", null, null, null);
        RunHandle handle = agent.run(spec);
        runIds.add(handle.runId());
        handle.collectAll();
    }
    assertEquals(runIds.size(), new HashSet<>(runIds).size());
}
```

## Offline behaviour

Model-name and run-ID tests are in-process. The agent loop uses
`Assumptions.assumeTrue(AncoraNative.AVAILABLE)`.
