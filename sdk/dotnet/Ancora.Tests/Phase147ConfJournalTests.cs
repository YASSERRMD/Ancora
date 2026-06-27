using System;
using System.Collections.Generic;
using System.Text.Json;
using System.Text.Json.Serialization;
using Xunit;

namespace Ancora.Tests;

public record JournalEntry147(int Seq, string Kind, string RunId);

public class Phase147ConfJournalTests
{
    private static readonly List<JournalEntry147> Journal =
    [
        new(0, "run_start", "conf-run-147"),
        new(1, "tool_call", "conf-run-147"),
        new(2, "tool_result", "conf-run-147"),
        new(3, "run_end", "conf-run-147"),
    ];

    [Fact]
    public void Journal_Has_Four_Entries()
    {
        Assert.Equal(4, Journal.Count);
    }

    [Fact]
    public void First_Entry_Is_Run_Start()
    {
        Assert.Equal("run_start", Journal[0].Kind);
    }

    [Fact]
    public void Last_Entry_Is_Run_End()
    {
        Assert.Equal("run_end", Journal[^1].Kind);
    }

    [Fact]
    public void Seq_Numbers_Are_Monotonic()
    {
        for (int i = 0; i < Journal.Count; i++)
            Assert.Equal(i, Journal[i].Seq);
    }

    [Fact]
    public void All_Entries_Share_RunId()
    {
        var runId = Journal[0].RunId;
        Assert.All(Journal, e => Assert.Equal(runId, e.RunId));
    }

    [Fact]
    public void Tool_Call_Precedes_Tool_Result()
    {
        var callIdx = Journal.FindIndex(e => e.Kind == "tool_call");
        var resultIdx = Journal.FindIndex(e => e.Kind == "tool_result");
        Assert.True(callIdx < resultIdx);
    }

    [Fact]
    public void Journal_Serializes_To_Json()
    {
        var json = JsonSerializer.Serialize(Journal);
        Assert.True(IsValidJson(json));
    }

    [Fact]
    public void Journal_RunId_Matches_Core_Fixture()
    {
        Assert.Equal("conf-run-147", Journal[0].RunId);
    }

    [Fact]
    public void Journal_Kind_Strings_Are_Lowercase_Underscore()
    {
        foreach (var entry in Journal)
        {
            Assert.Equal(entry.Kind, entry.Kind.ToLowerInvariant());
            Assert.DoesNotContain(" ", entry.Kind);
        }
    }

    [Fact]
    public void Journal_Entry_Roundtrip()
    {
        var entry = new JournalEntry147(0, "run_start", "r1");
        var json = JsonSerializer.Serialize(entry);
        var back = JsonSerializer.Deserialize<JournalEntry147>(json);
        Assert.Equal(entry, back);
    }

    private static bool IsValidJson(string s)
    {
        try { JsonDocument.Parse(s); return true; }
        catch { return false; }
    }
}
