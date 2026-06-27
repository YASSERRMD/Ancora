# Logging and Diagnostics (Java)

## Built-in native tracing

Ancora uses the Rust `tracing` crate internally. Control verbosity via:

```bash
ANCORA_LOG_LEVEL=debug java -jar myapp.jar
```

| Level | Description |
|-------|-------------|
| `error` | Fatal errors only |
| `warn` | Unexpected conditions (default) |
| `info` | Activity boundaries and run lifecycle |
| `debug` | Every activity recorded and replayed |
| `trace` | JNI boundary crossings |

## SLF4J bridge

When a `LoggerFactory` is provided via `RuntimeOptions`, Ancora forwards
native log messages to SLF4J:

```java
import org.slf4j.LoggerFactory;
import io.ancora.*;

var rt = new Runtime(new RuntimeOptions()
    .withLoggerFactory(LoggerFactory.getILoggerFactory()));
```

## Logback configuration

```xml
<!-- logback.xml -->
<logger name="io.ancora" level="DEBUG"/>
```

## MDC (Mapped Diagnostic Context)

Propagate the run ID to log lines:

```java
import org.slf4j.MDC;

MDC.put("ancora.runId", handle.runId());
try {
    for (var ev : handle.events()) { /* ... */ }
} finally {
    MDC.remove("ancora.runId");
}
```

## OpenTelemetry span events

Add log events to the current span:

```java
import io.opentelemetry.api.trace.Span;

var span = Span.current();
for (var ev : agent.run(spec, prompt).events()) {
    if (ev instanceof RunEvent.Token t) {
        span.addEvent("token", Attributes.of(AttributeKey.stringKey("token"), t.token()));
    }
}
```

## See also

- [Observability](observability.md)
- [Configuration](configuration.md)
