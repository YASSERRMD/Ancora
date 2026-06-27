# Java SDK API Reference

## `AncoraNative`

```java
public final class AncoraNative {
    public static final boolean AVAILABLE;   // true if native library loaded
}
```

## `Runtime (AutoCloseable)`

```java
public final class Runtime implements AutoCloseable {
    public Runtime() throws UnsatisfiedLinkError { }
    public Runtime(RuntimeOptions options) throws UnsatisfiedLinkError { }
    public void close() { }
}
```

## `Agent (AutoCloseable)`

```java
public final class Agent implements AutoCloseable {
    public Agent(Runtime runtime) { }

    public RunHandle run(AgentSpec spec, String prompt) { }
    public RunHandle run(AgentSpec spec, String prompt, RunOptions options) { }
    public RunHandle start(AgentSpec spec, String prompt) { }
    public RunHandle resume(String runId) { }
    public GraphHandle runGraph(GraphSpec graph, String prompt) { }
    public void close() { }
}
```

## `AgentSpec`

```java
public record AgentSpec(
    String model,
    String instructions,
    List<ToolSpec> tools,
    int maxTokens,
    float temperature
) {
    public static Builder builder() { }

    public static final class Builder {
        public Builder model(String model) { }
        public Builder instructions(String instructions) { }
        public Builder tools(List<ToolSpec> tools) { }
        public Builder policy(PolicySpec policy) { }
        public Builder mcpServers(List<String> servers) { }
        public Builder modelUrl(String url) { }
        public AgentSpec build() { }
    }
}
```

## `RunEvent` (sealed interface)

```java
public sealed interface RunEvent {
    record Started(String runId) implements RunEvent { }
    record Token(String token) implements RunEvent { }
    record ToolCall(String name, JsonNode input) implements RunEvent { }
    record Completed(String output, TokenUsage usage) implements RunEvent { }
    record Resumed(String runId) implements RunEvent { }
}
```

## `ToolSpec`

```java
public record ToolSpec(
    String name,
    String description,
    ToolInputSchema inputSchema,
    Function<JsonNode, String> fn
) { }
```

## `ToolInputSchema`

```java
public record ToolInputSchema(
    String type,
    Map<String, ToolInputProperty> properties,
    List<String> required
) { }
```

## `ToolInputProperty`

```java
public record ToolInputProperty(String type, String description) { }
```

## `PolicySpec`

```java
public record PolicySpec(
    List<String> allowRegions,
    List<String> denyProviders,
    int maxWriteTools
) { }
```

## `RunHandle`

```java
public final class RunHandle {
    public String runId() { }
    public RunStatus status() { }
    public String pauseReason() { }

    public Iterable<RunEvent> events() { }
    public void runUntilPause() { }
    public void resume(String payload) { }
    public void resumeBytes(byte[] payload) { }
}
```

## `SqliteStore` / `MemoryStore`

```java
public final class SqliteStore { public SqliteStore(String path) { } }
public final class MemoryStore { public boolean hasRun(String runId) { } }
```

## `GraphSpec`

```java
public record GraphSpec(List<GraphNode> nodes, List<GraphEdge> edges) { }
public record GraphNode(String id, AgentSpec spec) { }
public record GraphEdge(String from, String to) { }
```
