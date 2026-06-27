using System.Linq;
using System.Runtime.InteropServices;
using System.Threading.Tasks;
using Ancora;
using Ancora.Tests.Examples;
using Xunit;

namespace Ancora.Tests.Examples;

/// <summary>
/// Example: cost and OTEL trace.
///
/// Demonstrates wrapping an agent run in Span tracking to record event
/// counts, token estimates, and duration -- mirroring what an OTEL exporter
/// would consume.
/// </summary>
public sealed class CostOtelExampleTests
{
    [Fact]
    public void Span_Records_Name_And_Duration()
    {
        var s = new Span("agent.run");
        s.SetAttribute("run.id", "abc");
        var durationMs = s.EndMs();
        Assert.True(durationMs >= 0);
        Assert.Equal("abc", s.Attributes["run.id"]);
    }

    [Fact]
    public void TokenEstimator_Returns_At_Least_One()
    {
        Assert.Equal(1, TokenEstimator.EstimateTokens(""));
    }

    [Fact]
    public void TokenEstimator_Estimates_Four_Chars_Per_Token()
    {
        Assert.Equal(1, TokenEstimator.EstimateTokens("abcd"));
        Assert.Equal(2, TokenEstimator.EstimateTokens("abcde"));
        Assert.Equal(25, TokenEstimator.EstimateTokens(new string('x', 100)));
    }

    [Fact]
    public async Task Cost_Spans_Accumulate_Over_A_Run()
    {
        try
        {
            using var agent = new Agent();
            var root = new Span("agent.run");
            var events = await agent.Run(new AgentSpec("local-model", "Respond concisely.")).CollectAsync();

            var totalTokens = events.OfType<TokenEvent>().Sum(e => TokenEstimator.EstimateTokens(e.Text));
            root.SetAttribute("event.count", events.Count);
            root.SetAttribute("tokens.estimated", totalTokens);
            var durationMs = root.EndMs();

            Assert.True(durationMs >= 0);
            Assert.Equal(events.Count, (int)root.Attributes["event.count"]);
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public void Summary_Span_Can_Aggregate_Token_Total()
    {
        var summary = new Span("agent.summary");
        summary.SetAttribute("events", 5);
        summary.SetAttribute("tokens.estimated", 136);
        summary.EndMs();

        Assert.Equal(5, (int)summary.Attributes["events"]);
        Assert.Equal(136, (int)summary.Attributes["tokens.estimated"]);
    }
}
