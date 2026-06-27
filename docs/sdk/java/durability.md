# Durability and Restart Recovery (Java)

## Enable durability

```java
import io.ancora.*;

var store = new SqliteStore("/var/lib/myapp/journal.db");
var rt = new Runtime(new RuntimeOptions().withTransport(new StoringTransport(store)));
var agent = new Agent(rt);
```

With a `StoringTransport`, every run is journalled automatically. If the
JVM restarts mid-run, replay the journal to continue:

```java
var handle = agent.resume("run-abc-123");
for (var ev : handle.events()) {
    if (ev instanceof RunEvent.Completed c) System.out.println(c.output());
}
```

## Deterministic run IDs

```java
var handle = agent.run(spec, "Summarise the report.", new RunOptions().withRunId("report-summary-2026-06-28"));
```

Re-running with the same run ID replays completed activities from the journal.

## In-memory store (tests)

```java
var rt = new Runtime(new RuntimeOptions().withTransport(new StoringTransport(new MemoryStore())));
```

## Idempotency key templates

```java
var sendEmailTool = new ToolSpec(
    "send_email",
    "Send an email.",
    /* schema */,
    "send_email/{runId}/{seq}",   // idempotency key template
    args -> { /* send */ return "sent"; }
);
```

## See also

- [Observability](observability.md)
- [Durability concept](../../concepts/durability-and-replay.md)
