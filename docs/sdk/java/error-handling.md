# Error Handling (Java)

## Exception hierarchy

```
AncorException (RuntimeException)
├── NativeException            # JNI / native library failure
├── RunFailedException         # run terminated with an error event
├── PolicyViolationException   # policy rule blocked the run
├── TimeoutException           # run exceeded max runtime
└── JournalException           # journal read/write failure
```

## Catching run failures

```java
import io.ancora.*;

try {
    for (var ev : agent.run(spec, "What is 2+2?").events()) {
        if (ev instanceof RunEvent.Completed c) System.out.println(c.output());
    }
} catch (RunFailedException e) {
    System.err.printf("Run failed: %s (runId=%s)%n", e.getMessage(), e.getRunId());
}
```

## Retry on transient errors

```java
String runWithRetry(Agent agent, AgentSpec spec, String prompt, int maxAttempts) throws Exception {
    for (int attempt = 0; attempt < maxAttempts; attempt++) {
        try {
            for (var ev : agent.run(spec, prompt).events())
                if (ev instanceof RunEvent.Completed c) return c.output();
        } catch (RunFailedException e) {
            if (!e.isTransient() || attempt == maxAttempts - 1) throw e;
            Thread.sleep((long) Math.pow(2, attempt) * 1000);
        }
    }
    throw new IllegalStateException("unreachable");
}
```

## Thread-safe error collection

```java
import java.util.concurrent.*;

var errors = new CopyOnWriteArrayList<Exception>();

var tasks = prompts.stream().map(p -> CompletableFuture.supplyAsync(() -> {
    try {
        for (var ev : agent.run(spec, p).events())
            if (ev instanceof RunEvent.Completed c) return c.output();
        return "";
    } catch (Exception e) {
        errors.add(e);
        return "";
    }
})).toList();

var results = tasks.stream().map(CompletableFuture::join).toList();
if (!errors.isEmpty()) System.err.println(errors.size() + " runs failed");
```

## See also

- [Troubleshooting](troubleshooting.md)
- [Durability](durability.md)
