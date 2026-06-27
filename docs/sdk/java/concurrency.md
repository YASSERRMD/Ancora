# Concurrency (Java)

## Concurrent runs with virtual threads (Java 21+)

`Agent` is thread-safe. Multiple `agent.run()` calls can overlap:

```java
import io.ancora.*;
import java.util.concurrent.*;

try (var rt = new Runtime(); var agent = new Agent(rt)) {
    var spec = new AgentSpec("llama3", "Summarise.", List.of(), 1024, 0.7f);
    var prompts = List.of("Text A", "Text B", "Text C", "Text D");

    try (var executor = Executors.newVirtualThreadPerTaskExecutor()) {
        var futures = prompts.stream()
            .map(p -> executor.submit(() -> {
                for (var ev : agent.run(spec, p).events())
                    if (ev instanceof RunEvent.Completed c) return c.output();
                return "";
            }))
            .toList();

        for (var f : futures) System.out.println(f.get());
    }
}
```

## Thread pool with `CompletableFuture`

```java
var tasks = prompts.stream()
    .map(p -> CompletableFuture.supplyAsync(() -> {
        for (var ev : agent.run(spec, p).events())
            if (ev instanceof RunEvent.Completed c) return c.output();
        return "";
    }))
    .toList();

CompletableFuture.allOf(tasks.toArray(CompletableFuture[]::new)).join();
var results = tasks.stream().map(CompletableFuture::join).toList();
```

## Limiting concurrency with a semaphore

```java
var semaphore = new Semaphore(4);

var tasks = prompts.stream()
    .map(p -> CompletableFuture.supplyAsync(() -> {
        semaphore.acquire();
        try {
            for (var ev : agent.run(spec, p).events())
                if (ev instanceof RunEvent.Completed c) return c.output();
            return "";
        } finally {
            semaphore.release();
        }
    }))
    .toList();
```

## Note on `Runtime` fork-safety

`Runtime` must be created after any `fork()`. In practice, prefer one `Runtime`
per JVM process and share it across threads.

## See also

- [Streaming](streaming.md)
- [Multi-agent graphs](multi-agent.md)
