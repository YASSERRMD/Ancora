# Structured Output (.NET)

Force the agent to return a JSON-serialisable record instead of raw text.

## Define the output type

```csharp
using System.Text.Json.Serialization;

public record AnalysisResult(
    [property: JsonPropertyName("headline")] string Headline,
    [property: JsonPropertyName("sentiment")] string Sentiment,
    [property: JsonPropertyName("confidence")] double Confidence
);
```

## Pass the schema to AgentSpec

```csharp
using Ancora;
using System.Text.Json;

var rt = new Runtime();
await using var agent = new Agent(rt);

var spec = new AgentSpec
{
    Model = "llama3",
    Instructions = "Analyse the sentiment of the user message.",
    OutputSchema = JsonSchemaGenerator.FromType<AnalysisResult>(),
};

await foreach (var ev in agent.Run(spec, "Ancora makes development simple!").Events())
{
    if (ev is CompletedEvent completed)
    {
        var result = JsonSerializer.Deserialize<AnalysisResult>(completed.Output);
        Console.WriteLine($"{result!.Headline} ({result.Sentiment}, {result.Confidence:P0})");
    }
}
```

## Nested types

```csharp
public record Tag(string Name, double Score);

public record Report(
    [property: JsonPropertyName("summary")] string Summary,
    [property: JsonPropertyName("tags")] List<Tag> Tags
);

var spec = new AgentSpec
{
    Model = "llama3",
    Instructions = "Tag this text and return JSON.",
    OutputSchema = JsonSchemaGenerator.FromType<Report>(),
};
```

## See also

- [Tools](tools.md)
- [API reference](api-reference.md)
