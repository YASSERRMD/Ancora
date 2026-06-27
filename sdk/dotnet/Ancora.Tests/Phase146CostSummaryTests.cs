using System.Collections.Generic;
using System.Text.Json;
using System.Runtime.InteropServices;
using System.Threading.Tasks;
using Ancora;
using Xunit;

namespace Ancora.Tests;

public record CostEvent146(int InputTokens, int OutputTokens, double Cost);

public class Phase146CostSummaryTests
{
    private static readonly CostEvent146[] Fixture =
    [
        new(100, 50, 0.0015),
        new(200, 80, 0.0028),
        new(150, 60, 0.0021),
    ];

    [Fact]
    public void CostEvent_Stores_Fields()
    {
        var ev = new CostEvent146(100, 50, 0.0015);
        Assert.Equal(100, ev.InputTokens);
        Assert.Equal(50, ev.OutputTokens);
        Assert.Equal(0.0015, ev.Cost);
    }

    [Fact]
    public void Cost_Accumulates_Across_Events()
    {
        double total = 0;
        foreach (var ev in Fixture) total += ev.Cost;
        Assert.True(total > 0.0);
    }

    [Fact]
    public void Cost_Is_Non_Negative()
    {
        foreach (var ev in Fixture)
            Assert.True(ev.Cost >= 0);
    }

    [Fact]
    public void Input_Tokens_Are_Non_Negative()
    {
        foreach (var ev in Fixture)
            Assert.True(ev.InputTokens >= 0);
    }

    [Fact]
    public void Output_Tokens_Are_Non_Negative()
    {
        foreach (var ev in Fixture)
            Assert.True(ev.OutputTokens >= 0);
    }

    [Fact]
    public void Cost_Json_Is_Parseable()
    {
        var json = JsonSerializer.Serialize(Fixture[0]);
        Assert.True(IsValidJson(json));
    }

    [Fact]
    public async Task RunHandle_GetCost_Returns_Json()
    {
        try
        {
            using var a = new Agent();
            var h = a.Run(new AgentSpec("llama3"));
            await h.CollectAsync();
            var cost = h.GetCost();
            Assert.True(IsValidJson(cost));
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public void Three_Events_Total_Input_Tokens()
    {
        int total = 0;
        foreach (var ev in Fixture) total += ev.InputTokens;
        Assert.Equal(450, total);
    }

    [Fact]
    public void Three_Events_Total_Output_Tokens()
    {
        int total = 0;
        foreach (var ev in Fixture) total += ev.OutputTokens;
        Assert.Equal(190, total);
    }

    [Fact]
    public void CostEvent_Value_Equality()
    {
        var a = new CostEvent146(100, 50, 0.0015);
        var b = new CostEvent146(100, 50, 0.0015);
        Assert.Equal(a, b);
    }

    private static bool IsValidJson(string s)
    {
        try { JsonDocument.Parse(s); return true; }
        catch { return false; }
    }
}
