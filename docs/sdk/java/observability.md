# Observability and Cost (Java)

## In-process token tracking

```java
import io.ancora.*;

int totalTokens = 0;
for (var ev : agent.run(spec, "What is 2+2?").events()) {
    if (ev instanceof RunEvent.Token t) {
        totalTokens += (int) Math.ceil(t.token().length() / 4.0);
    } else if (ev instanceof RunEvent.Completed c) {
        System.out.printf("Input: %d, Output: %d%n",
            c.usage().inputTokens(), c.usage().outputTokens());
    }
}
System.out.println("Estimated output tokens: " + totalTokens);
```

## OpenTelemetry export

```xml
<!-- pom.xml -->
<dependency>
    <groupId>io.opentelemetry</groupId>
    <artifactId>opentelemetry-sdk</artifactId>
    <version>1.37.0</version>
</dependency>
<dependency>
    <groupId>io.opentelemetry</groupId>
    <artifactId>opentelemetry-exporter-otlp</artifactId>
    <version>1.37.0</version>
</dependency>
```

```java
import io.opentelemetry.api.*;
import io.opentelemetry.api.trace.*;
import io.opentelemetry.sdk.*;
import io.opentelemetry.sdk.trace.*;
import io.opentelemetry.exporter.otlp.trace.*;

var exporter = OtlpGrpcSpanExporter.builder()
    .setEndpoint("http://localhost:4317").build();

var sdk = OpenTelemetrySdk.builder()
    .setTracerProvider(SdkTracerProvider.builder()
        .addSpanProcessor(BatchSpanProcessor.builder(exporter).build()).build())
    .build();

var tracer = sdk.getTracer("ancora");
var span = tracer.spanBuilder("agent-run").startSpan();

try (var scope = span.makeCurrent()) {
    for (var ev : agent.run(spec, "What is 2+2?").events()) {
        if (ev instanceof RunEvent.Completed c) {
            span.setAttribute("ancora.model", spec.model());
            span.setAttribute("ancora.output_tokens", c.usage().outputTokens());
        }
    }
} finally {
    span.end();
}
```

## See also

- [Durability](durability.md)
- [Policy](policy.md)
