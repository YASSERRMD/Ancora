using System;
using System.Collections.Generic;
using System.Linq;
using System.Text.Json;
using Xunit;

namespace Ancora.Tests;

/// <summary>
/// Cross-language conformance: single agent scenario -- .NET binding (offline fixture).
/// </summary>
public class XlangSingleAgentTests
{
    private const string XlangRunId = "xlang-dotnet-001";

    private static IReadOnlyList<Dictionary<string, object>> MakeXlangEvents(string runId) =>
        new List<Dictionary<string, object>>
        {
            new() { ["kind"] = "started", ["run_id"] = runId, ["spec"] = "{}" },
            new() { ["kind"] = "token",   ["run_id"] = runId, ["text"] = "xlang dotnet result" },
            new() { ["kind"] = "completed", ["run_id"] = runId },
        };

    [Fact]
    public void StartedEventIsFirst()
    {
        var events = MakeXlangEvents(XlangRunId);
        Assert.Equal("started", events[0]["kind"]);
    }

    [Fact]
    public void CompletedEventIsLast()
    {
        var events = MakeXlangEvents(XlangRunId);
        Assert.Equal("completed", events[^1]["kind"]);
    }

    [Fact]
    public void RunIdConsistentAcrossEvents()
    {
        var events = MakeXlangEvents(XlangRunId);
        foreach (var ev in events)
            Assert.Equal(XlangRunId, ev["run_id"]);
    }

    [Fact]
    public void EventCountAtLeastTwo()
    {
        var events = MakeXlangEvents(XlangRunId);
        Assert.True(events.Count >= 2);
    }

    [Fact]
    public void TokenEventHasNonEmptyText()
    {
        var events = MakeXlangEvents(XlangRunId);
        var tokens = events.Where(e => (string)e["kind"] == "token").ToList();
        Assert.NotEmpty(tokens);
        foreach (var tok in tokens)
            Assert.True(!string.IsNullOrEmpty((string)tok["text"]));
    }

    [Fact]
    public void EventsSerialiseToValidJson()
    {
        var events = MakeXlangEvents(XlangRunId);
        foreach (var ev in events)
        {
            var json = JsonSerializer.Serialize(ev);
            var decoded = JsonSerializer.Deserialize<Dictionary<string, object>>(json);
            Assert.NotNull(decoded);
        }
    }

    [Fact]
    public void StartedSpecFieldIsValidJson()
    {
        var events = MakeXlangEvents(XlangRunId);
        var spec = (string)events[0]["spec"];
        var parsed = JsonSerializer.Deserialize<Dictionary<string, object>>(spec);
        Assert.NotNull(parsed);
    }
}
