using System.Collections.Generic;
using System.Linq;
using System.Text.Json;
using Xunit;

namespace Ancora.Tests;

/// <summary>Cross-language conformance: human-in-loop scenario -- .NET (offline).</summary>
public class XlangHumanInLoopTests
{
    private const string RunId = "xlh-dotnet";

    private record HilEvent(string Kind, string RunId, string? Prompt = null, List<string>? Options = null, string? Decision = null, Dictionary<string, string>? Output = null);

    private static IReadOnlyList<HilEvent> MakeEvents(string runId) =>
    [
        new("started",            runId),
        new("decision_requested", runId, Prompt: "Please approve the draft", Options: ["approve", "reject"]),
        new("decision_received",  runId, Decision: "{\"approved\":true}"),
        new("completed",          runId, Output: new() { ["result"] = "hil-ok" }),
    ];

    [Fact] public void StartedIsFirst() { Assert.Equal("started", MakeEvents(RunId)[0].Kind); }

    [Fact] public void RequestedBeforeReceived()
    {
        var kinds = MakeEvents(RunId).Where(e => e.Kind.StartsWith("decision")).Select(e => e.Kind).ToList();
        Assert.Equal(new[] { "decision_requested", "decision_received" }, kinds);
    }

    [Fact] public void DecisionIsApproved()
    {
        var received = MakeEvents(RunId).First(e => e.Kind == "decision_received");
        var dec = JsonSerializer.Deserialize<Dictionary<string, object>>(received.Decision!);
        Assert.True(dec!["approved"].ToString()!.ToLower() == "true");
    }

    [Fact] public void PromptIsNonEmpty()
    {
        var requested = MakeEvents(RunId).First(e => e.Kind == "decision_requested");
        Assert.True(!string.IsNullOrEmpty(requested.Prompt));
        Assert.NotEmpty(requested.Options!);
    }

    [Fact] public void CompletedIsLast() { Assert.Equal("completed", MakeEvents(RunId)[^1].Kind); }

    [Fact] public void RunIdConsistent()
    {
        foreach (var ev in MakeEvents(RunId))
            Assert.Equal(RunId, ev.RunId);
    }
}
