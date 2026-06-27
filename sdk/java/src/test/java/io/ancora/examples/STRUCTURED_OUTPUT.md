# Structured Output Example

Demonstrates deriving a `ToolInputSchema` from a Java record and validating
that the schema and JSON round-trip correctly using Jackson.

## What it tests

- A Java `record` with `@JsonProperty` annotations serializes with the
  correct property names
- `ToolInputSchema` holds the expected type, properties, and required fields
- JSON serialization and deserialization round-trips cleanly

## Pattern

```java
record AnalysisResult(
    @JsonProperty("summary")   String summary,
    @JsonProperty("sentiment") String sentiment,
    @JsonProperty("score")     double score
) {}

var schema = new ToolInputSchema(
    "object",
    Map.of(
        "summary",   new ToolInputProperty("string", "Brief summary"),
        "sentiment", new ToolInputProperty("string", "positive, neutral, or negative"),
        "score",     new ToolInputProperty("number", "Confidence 0-1")
    ),
    List.of("summary", "sentiment", "score")
);

String json = mapper.writeValueAsString(new AnalysisResult("Good", "positive", 0.9));
assertTrue(json.contains("\"summary\""));
```

## Offline behaviour

Pure Jackson serialization tests run entirely in-process. Agent run tests
skip when the native library is absent.
