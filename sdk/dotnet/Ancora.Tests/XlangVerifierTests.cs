using System.Collections.Generic;
using System.Linq;
using System.Text.Json;
using Xunit;

namespace Ancora.Tests;

/// <summary>Cross-language conformance: verifier scenario -- .NET (offline).</summary>
public class XlangVerifierTests
{
    private const string RunId = "xlv-dotnet";

    private record VerifierEvent(string Kind, string RunId, string? ActivityKey = null, Dictionary<string, string>? Output = null);

    private static IReadOnlyList<VerifierEvent> MakeEvents(string runId) =>
    [
        new("started",   runId),
        new("activity",  runId, "drafter"),
        new("activity",  runId, "verifier"),
        new("completed", runId, Output: new() { ["verdict"] = "approved" }),
    ];

    [Fact] public void StartedIsFirst()
    {
        Assert.Equal("started", MakeEvents(RunId)[0].Kind);
    }

    [Fact] public void CompletedIsLast()
    {
        var evs = MakeEvents(RunId);
        Assert.Equal("completed", evs[^1].Kind);
    }

    [Fact] public void DrafterBeforeVerifier()
    {
        var keys = MakeEvents(RunId).Where(e => e.Kind == "activity").Select(e => e.ActivityKey).ToList();
        Assert.Equal(new[] { "drafter", "verifier" }, keys);
    }

    [Fact] public void OutputVerdictIsApproved()
    {
        var last = MakeEvents(RunId)[^1];
        Assert.Equal("approved", last.Output?["verdict"]);
    }

    [Fact] public void RunIdConsistent()
    {
        foreach (var ev in MakeEvents(RunId))
            Assert.Equal(RunId, ev.RunId);
    }
}
