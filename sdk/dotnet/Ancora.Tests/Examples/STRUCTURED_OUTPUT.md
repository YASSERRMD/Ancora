# Structured Output Example

Demonstrates deriving a JSON Schema from a C# record, building a
`ToolInputSchema`, and validating that the schema round-trips through
`System.Text.Json`.

## What it tests

- A C# `record` with `[JsonPropertyName]` attributes maps to the expected
  property names
- `ToolInputSchema` holds the correct type, properties, and required list
- `AgentSpec` with an `OutputSchema` runs without error

## Pattern

```csharp
record AnalysisResult(
    [property: JsonPropertyName("summary")]   string Summary,
    [property: JsonPropertyName("sentiment")] string Sentiment,
    [property: JsonPropertyName("score")]     double Score
);

var schema = new ToolInputSchema(
    Type: "object",
    Properties: new Dictionary<string, ToolInputProperty>
    {
        ["summary"]   = new("string", "Brief summary"),
        ["sentiment"] = new("string", "positive, neutral, or negative"),
        ["score"]     = new("number", "Confidence 0-1"),
    },
    Required: new List<string> { "summary", "sentiment", "score" }
);

var json = JsonSerializer.Serialize(schema);
var parsed = JsonSerializer.Deserialize<ToolInputSchema>(json)!;
Assert.Equal("object", parsed.Type);
```

## Offline behaviour

Agent run tests catch `DllNotFoundException` when the native library is
absent.
