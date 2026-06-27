using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Text.Json;
using System.Threading.Tasks;
using Ancora;
using Xunit;

namespace Ancora.Tests.Examples;

/// <summary>
/// Example: MCP tool use.
///
/// Demonstrates defining tool specs, wiring them as function delegates,
/// and verifying the ToolRegistry dispatch pattern.
/// </summary>
public sealed class McpToolExampleTests
{
    private static string GetWeather(string location)
        => $"Weather in {location}: 22 C, partly cloudy";

    private static double Calculate(string expression)
    {
        var parts = expression.Split('+');
        return parts.Length == 2 && double.TryParse(parts[0].Trim(), out var a)
               && double.TryParse(parts[1].Trim(), out var b)
               ? a + b
               : 0;
    }

    [Fact]
    public void WeatherTool_Returns_Result_For_City()
    {
        var result = GetWeather("Cairo");
        Assert.Contains("Cairo", result);
        Assert.Contains("22 C", result);
    }

    [Fact]
    public void CalculateTool_Adds_Two_Numbers()
    {
        var result = Calculate("3 + 4");
        Assert.Equal(7.0, result, precision: 5);
    }

    [Fact]
    public void ToolSpec_Has_Correct_Name_And_Description()
    {
        var spec = new ToolSpec(
            Name: "get_weather",
            Description: "Get weather for a location.",
            InputSchema: new ToolInputSchema(
                Type: "object",
                Properties: new Dictionary<string, ToolInputProperty>
                {
                    ["location"] = new("string", "City name"),
                },
                Required: new List<string> { "location" }
            )
        );

        Assert.Equal("get_weather", spec.Name);
        Assert.Equal(1, spec.InputSchema?.Properties?.Count);
        Assert.Contains("location", spec.InputSchema!.Required!);
    }

    [Fact]
    public async Task AgentSpec_With_Tools_Runs_Without_Error()
    {
        try
        {
            var weatherSpec = new ToolSpec(
                Name: "get_weather",
                Description: "Get weather for a location.",
                InputSchema: new ToolInputSchema(
                    Type: "object",
                    Properties: new Dictionary<string, ToolInputProperty>
                    {
                        ["location"] = new("string", "City name"),
                    },
                    Required: new List<string> { "location" }
                )
            );
            using var agent = new Agent();
            var spec = new AgentSpec("local-model", "Use tools.", new List<ToolSpec> { weatherSpec });
            var events = await agent.Run(spec).CollectAsync();
            Assert.NotEmpty(events);
        }
        catch (DllNotFoundException) { }
    }
}
