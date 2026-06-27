# Streaming (Java)

Iterate over run events as they arrive using the `Iterable<RunEvent>` API.

## Basic streaming

```java
import io.ancora.*;

try (var rt = new Runtime(); var agent = new Agent(rt)) {
    var spec = new AgentSpec("llama3", "Tell a short story.", List.of(), 1024, 0.7f);

    for (var ev : agent.run(spec, "Once upon a time...").events()) {
        if (ev instanceof RunEvent.Token t) {
            System.out.print(t.token());
            System.out.flush();
        }
    }
    System.out.println();
}
```

## Accumulating tokens

```java
var sb = new StringBuilder();

for (var ev : agent.run(spec, prompt).events()) {
    if (ev instanceof RunEvent.Token t) sb.append(t.token());
}

String fullText = sb.toString();
```

## Event types

| Type | Method | Description |
|------|--------|-------------|
| `RunEvent.Started` | `runId()` | Run has begun |
| `RunEvent.Token` | `token()` | One model output token |
| `RunEvent.ToolCall` | `name()`, `input()` | Agent called a tool |
| `RunEvent.Completed` | `output()`, `usage()` | Run finished |
| `RunEvent.Resumed` | `runId()` | Run resumed after pause |

## Streaming with an executor

```java
import java.util.concurrent.*;

var queue = new LinkedBlockingQueue<RunEvent>();

var executor = Executors.newVirtualThreadPerTaskExecutor();
executor.submit(() -> {
    for (var ev : agent.run(spec, prompt).events()) {
        queue.put(ev);
    }
    queue.put(RunEvent.END_SENTINEL);   // sentinel to signal done
});

RunEvent ev;
while ((ev = queue.take()) != RunEvent.END_SENTINEL) {
    if (ev instanceof RunEvent.Token t) System.out.print(t.token());
}
```

## See also

- [Human-in-the-loop](human-in-the-loop.md)
- [Quickstart](quickstart.md)
