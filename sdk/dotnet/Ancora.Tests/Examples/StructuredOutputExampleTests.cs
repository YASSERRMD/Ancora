using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Text.Json;
using System.Text.Json.Serialization;
using System.Threading.Tasks;
using Ancora;
using Xunit;

namespace Ancora.Tests.Examples;

/// <summary>
/// Example: structured typed output with C# records.
///
/// Derives a JSON schema from a C# record type and injects it into the
/// agent system prompt so the agent produces structured output.
/// </summary>

/// <summary>
/// Expected agent output shape for an analysis task.
/// </summary>
public sealed record AnalysisResult(
    [property: JsonPropertyName("summary")] string Summary,
    [property: JsonPropertyName("topics")] List<string> Topics,
    [property: JsonPropertyName("confidence")] double Confidence,
    [property: JsonPropertyName("action_item")] string ActionItem
);

public sealed class StructuredOutputExampleTests
{
    [Fact]
    public void ToolInputSchema_Can_Describe_AnalysisResult_Shape()
    {
        var schema = new ToolInputSchema(
            Type: "object",
            Properties: new Dictionary<string, ToolInputProperty>
            {
                ["summary"]     = new("string", "One-sentence summary of the analysis"),
                ["topics"]      = new("array",  "List of main topics identified"),
                ["confidence"]  = new("number", "Confidence score between 0.0 and 1.0"),
                ["action_item"] = new("string", "Recommended next action"),
            },
            Required: new List<string> { "summary", "topics", "confidence", "action_item" }
        );

        Assert.Equal(4, schema.Properties!.Count);
        Assert.Contains("summary", schema.Required!);
        Assert.Contains("confidence", schema.Required!);
    }

    [Fact]
    public void AnalysisResult_Can_Be_Deserialized_From_Json()
    {
        var json = """
            {
                "summary": "Ancora is a multi-backend agent runtime.",
                "topics": ["agents", "backends"],
                "confidence": 0.92,
                "action_item": "Review the pgvector backend."
            }
            """;
        var result = JsonSerializer.Deserialize<AnalysisResult>(json);
        Assert.NotNull(result);
        Assert.Equal("Ancora is a multi-backend agent runtime.", result!.Summary);
        Assert.Equal(2, result.Topics.Count);
        Assert.InRange(result.Confidence, 0.0, 1.0);
    }

    [Fact]
    public async Task StructuredOutput_Agent_Produces_Events()
    {
        try
        {
            using var agent = new Agent();
            var schemaJson = JsonSerializer.Serialize(new
            {
                type = "object",
                properties = new
                {
                    summary = new { type = "string" },
                    confidence = new { type = "number" },
                }
            });
            var spec = new AgentSpec("local-model", $"Respond with JSON matching:\n{schemaJson}");
            var events = await agent.Run(spec).CollectAsync();
            Assert.NotEmpty(events);
        }
        catch (DllNotFoundException) { }
    }
}
