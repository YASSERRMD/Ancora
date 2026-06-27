# Structured Output (Java)

Force the agent to return a JSON-deserialisable record instead of raw text.

## Define the output type

```java
import com.fasterxml.jackson.annotation.JsonProperty;

public record AnalysisResult(
    @JsonProperty("headline") String headline,
    @JsonProperty("sentiment") String sentiment,
    @JsonProperty("confidence") double confidence
) {}
```

## Pass the schema to AgentSpec

```java
import io.ancora.*;
import com.fasterxml.jackson.databind.ObjectMapper;
import java.util.List;

var mapper = new ObjectMapper();
var rt = new Runtime();
var agent = new Agent(rt);

var spec = new AgentSpec(
    "llama3",
    "Analyse the sentiment of the user message.",
    List.of(),
    4096, 0.3f
);

for (var ev : agent.run(spec, "Ancora makes development simple!").events()) {
    if (ev instanceof RunEvent.Completed c) {
        var result = mapper.readValue(c.output(), AnalysisResult.class);
        System.out.printf("%s (%s, %.0f%%)%n",
            result.headline(), result.sentiment(), result.confidence() * 100);
    }
}
```

## Nested types

```java
public record Tag(@JsonProperty("name") String name, @JsonProperty("score") double score) {}

public record Report(
    @JsonProperty("summary") String summary,
    @JsonProperty("tags") List<Tag> tags
) {}
```

## See also

- [Tools](tools.md)
- [API reference](api-reference.md)
