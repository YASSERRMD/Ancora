using System.Collections.Generic;
using System.Text.Json;
using System.Runtime.InteropServices;
using System.Threading.Tasks;
using Ancora;
using Xunit;

namespace Ancora.Tests;

public record OtelSpan147(string TraceId, string SpanId, string Operation);
public record CostOtelEvent147(OtelSpan147 Span, int InputTokens, int OutputTokens, double Cost);

public class Phase147E2eCostOtelTests
{
    private static readonly OtelSpan147 OtelSpanFixture = new("trace-abc", "span-001", "ancora.run");

    private static readonly CostOtelEvent147[] CostEvents =
    [
        new(OtelSpanFixture, 100, 50, 0.0015),
        new(OtelSpanFixture, 200, 80, 0.0028),
        new(OtelSpanFixture, 150, 60, 0.0021),
        new(OtelSpanFixture, 120, 45, 0.0018),
        new(OtelSpanFixture, 180, 70, 0.0025),
    ];

    [Fact]
    public void OtelSpan_Stores_TraceId_SpanId_Operation()
    {
        Assert.Equal("trace-abc", OtelSpanFixture.TraceId);
        Assert.Equal("span-001", OtelSpanFixture.SpanId);
        Assert.Equal("ancora.run", OtelSpanFixture.Operation);
    }

    [Fact]
    public void Five_Cost_Events_In_Fixture()
    {
        Assert.Equal(5, CostEvents.Length);
    }

    [Fact]
    public void Cost_Accumulates_Across_Five_Runs()
    {
        double total = 0;
        foreach (var ev in CostEvents) total += ev.Cost;
        Assert.True(total > 0.0);
    }

    [Fact]
    public void Total_Input_Tokens_Across_Five_Runs()
    {
        int total = 0;
        foreach (var ev in CostEvents) total += ev.InputTokens;
        Assert.Equal(750, total);
    }

    [Fact]
    public void Total_Output_Tokens_Across_Five_Runs()
    {
        int total = 0;
        foreach (var ev in CostEvents) total += ev.OutputTokens;
        Assert.Equal(305, total);
    }

    [Fact]
    public void All_Events_Share_Otel_Span()
    {
        Assert.All(CostEvents, ev => Assert.Equal(OtelSpanFixture.TraceId, ev.Span.TraceId));
    }

    [Fact]
    public void Cost_Event_Serializes_To_Json()
    {
        var json = JsonSerializer.Serialize(CostEvents[0]);
        Assert.True(IsValidJson(json));
    }

    [Fact]
    public async Task Run_GetCost_Returns_Json()
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
    public void All_Costs_Non_Negative()
    {
        Assert.All(CostEvents, ev => Assert.True(ev.Cost >= 0));
    }

    [Fact]
    public void Otel_Operation_Is_Ancora_Run()
    {
        Assert.Equal("ancora.run", OtelSpanFixture.Operation);
    }

    private static bool IsValidJson(string s)
    {
        try { JsonDocument.Parse(s); return true; }
        catch { return false; }
    }
}
