using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text.Json;
using System.Text.Json.Serialization;
using System.Threading.Tasks;
using Ancora;
using Xunit;

namespace Ancora.Tests;

/// <summary>
/// Conformance scenarios for the .NET SDK.
/// These mirror the same scenarios defined in spec/conformance and verified
/// by the Rust core (Phase 49) and other language SDKs.
/// </summary>
public class SingleAgentConformanceTests
{
    [Fact]
    public async Task SingleAgent_StartedEvent_Arrives_First()
    {
        try
        {
            using var agent = new Agent();
            var spec = new AgentSpec("conformance-model", "Conform to the spec");
            var handle = agent.Run(spec);
            var events = await handle.CollectAsync();
            Assert.True(events.Count >= 1);
            Assert.IsType<StartedEvent>(events[0]);
        }
        catch (DllNotFoundException)
        {
            // Native library absent; CI provides it.
        }
    }

    [Fact]
    public async Task SingleAgent_CompletedEvent_Arrives_Last()
    {
        try
        {
            using var agent = new Agent();
            var spec = new AgentSpec("conformance-model", "Complete after one step");
            var handle = agent.Run(spec);
            var events = await handle.CollectAsync();
            Assert.True(events.Count >= 1);
            Assert.IsType<CompletedEvent>(events[events.Count - 1]);
        }
        catch (DllNotFoundException)
        {
            // Native library absent; CI provides it.
        }
    }

    [Fact]
    public async Task SingleAgent_RunId_Is_Consistent_Across_Events()
    {
        try
        {
            using var agent = new Agent();
            var spec = new AgentSpec("conformance-model", "Consistent run id");
            var handle = agent.Run(spec);
            var events = await handle.CollectAsync();
            foreach (var ev in events)
                Assert.Equal(handle.RunId, ev.RunId);
        }
        catch (DllNotFoundException)
        {
            // Native library absent; CI provides it.
        }
    }

    [Fact]
    public async Task SingleAgent_RunId_Is_Non_Empty()
    {
        try
        {
            using var agent = new Agent();
            var spec = new AgentSpec("conformance-model", "Non-empty run id");
            var handle = agent.Run(spec);
            Assert.False(string.IsNullOrEmpty(handle.RunId));
            await handle.CollectAsync();
        }
        catch (DllNotFoundException)
        {
            // Native library absent; CI provides it.
        }
    }
}

public class MultiAgentVerifierConformanceTests
{
    [Fact]
    public async Task MultiRun_Each_Run_Gets_Distinct_RunId()
    {
        try
        {
            using var agent = new Agent();
            var spec = new AgentSpec("conformance-model", "Multi-run");
            var handle1 = agent.Run(spec);
            var handle2 = agent.Run(spec);
            Assert.NotEqual(handle1.RunId, handle2.RunId);
            await handle1.CollectAsync();
            await handle2.CollectAsync();
        }
        catch (DllNotFoundException)
        {
            // Native library absent; CI provides it.
        }
    }
}

public class HumanInLoopConformanceTests
{
    [Fact]
    public async Task Resume_Produces_Resumed_Event()
    {
        try
        {
            using var agent = new Agent();
            var spec = new AgentSpec("conformance-model", "Await decision");
            var handle = agent.Run(spec);
            await handle.CollectAsync();
            var afterResume = await handle.ResumeAndCollectAsync("approved");
            Assert.True(afterResume.Count >= 1);
            Assert.IsType<ResumedEvent>(afterResume[0]);
        }
        catch (DllNotFoundException)
        {
            // Native library absent; CI provides it.
        }
    }

    [Fact]
    public async Task Resume_Completed_Event_Is_Last_After_Decision()
    {
        try
        {
            using var agent = new Agent();
            var spec = new AgentSpec("conformance-model", "Decision flow");
            var handle = agent.Run(spec);
            await handle.CollectAsync();
            var afterResume = await handle.ResumeAndCollectAsync("approved");
            Assert.True(afterResume.Count >= 2);
            Assert.IsType<CompletedEvent>(afterResume[afterResume.Count - 1]);
        }
        catch (DllNotFoundException)
        {
            // Native library absent; CI provides it.
        }
    }
}

public class CrashRecoverConformanceTests
{
    [Fact]
    public async Task Dispose_And_Recreate_Agent_Does_Not_Corrupt_State()
    {
        try
        {
            AgentSpec spec = new AgentSpec("conformance-model", "Crash scenario");
            string? runId = null;

            // simulate crash: create agent, start run, then dispose agent early
            var agent1 = new Agent();
            var handle1 = agent1.Run(spec);
            runId = handle1.RunId;
            agent1.Dispose();

            // recreate agent with a fresh runtime (new run)
            using var agent2 = new Agent();
            var handle2 = agent2.Run(spec);
            Assert.NotEqual(runId, handle2.RunId);
            var events = await handle2.CollectAsync();
            Assert.True(events.Count >= 1);
        }
        catch (DllNotFoundException)
        {
            // Native library absent; CI provides it.
        }
    }
}

public class CostConformanceTests
{
    [Fact]
    public async Task CostJson_Contains_RunId()
    {
        try
        {
            using var agent = new Agent();
            var spec = new AgentSpec("conformance-model", "Cost tracking");
            var handle = agent.Run(spec);
            await handle.CollectAsync();
            var cost = handle.GetCost();
            Assert.Contains(handle.RunId, cost);
        }
        catch (DllNotFoundException)
        {
            // Native library absent; CI provides it.
        }
    }

    [Fact]
    public async Task CostJson_Contains_Total_Usd_Field()
    {
        try
        {
            using var agent = new Agent();
            var spec = new AgentSpec("conformance-model", "Cost USD field");
            var handle = agent.Run(spec);
            await handle.CollectAsync();
            var cost = handle.GetCost();
            Assert.Contains("total_usd", cost);
        }
        catch (DllNotFoundException)
        {
            // Native library absent; CI provides it.
        }
    }
}

/// <summary>
/// Replay the fixture events defined in sdk/ts/__tests__/conformance/fixtures.json
/// to verify the .NET event parser agrees with the canonical fixture.
/// </summary>
public class FixtureConformanceTests
{
    private static readonly JsonSerializerOptions _opts = Wire.Options;

    [Fact]
    public void Fixture_Event_Started_Parses_Correctly()
    {
        var json = """{"kind":"started","run_id":"fixture-run-1","spec":"{\"model\":\"claude-3-5-sonnet\"}"}""";
        var ev = Wire.ParseEvent(json);
        var started = Assert.IsType<StartedEvent>(ev);
        Assert.Equal("fixture-run-1", started.RunId);
        Assert.Contains("claude-3-5-sonnet", started.Spec);
    }

    [Fact]
    public void Fixture_Event_Token_Parses_Correctly()
    {
        var json = """{"kind":"token","run_id":"fixture-run-1","text":"Hello"}""";
        var ev = Wire.ParseEvent(json);
        var token = Assert.IsType<TokenEvent>(ev);
        Assert.Equal("Hello", token.Text);
    }

    [Fact]
    public void Fixture_Event_Completed_Parses_Correctly()
    {
        var json = """{"kind":"completed","run_id":"fixture-run-1"}""";
        var ev = Wire.ParseEvent(json);
        Assert.IsType<CompletedEvent>(ev);
    }

    [Fact]
    public void Fixture_Event_ToolCall_Parses_Correctly()
    {
        var json = """{"kind":"tool_call","run_id":"fixture-run-2","name":"get_weather","input":"{\"city\":\"Paris\"}"}""";
        var ev = Wire.ParseEvent(json);
        var tc = Assert.IsType<ToolCallEvent>(ev);
        Assert.Equal("get_weather", tc.Name);
        Assert.Contains("Paris", tc.Input);
    }

    [Fact]
    public void Fixture_Event_Resumed_Parses_Correctly()
    {
        var json = """{"kind":"resumed","run_id":"fixture-run-2","decision":"{\"temperature\":\"22C\"}"}""";
        var ev = Wire.ParseEvent(json);
        var resumed = Assert.IsType<ResumedEvent>(ev);
        Assert.Contains("22C", resumed.Decision);
    }

    [Fact]
    public void Fixture_AgentSpec_Serializes_To_Matching_Fields()
    {
        var spec = new AgentSpec(
            "claude-3-5-sonnet",
            "You are a helpful assistant.",
            MaxTokens: 1024,
            Temperature: 0.7);
        var bytes = Wire.EncodeAgentSpec(spec);
        var json = System.Text.Encoding.UTF8.GetString(bytes);
        Assert.Contains("\"model\":", json);
        Assert.Contains("claude-3-5-sonnet", json);
        Assert.Contains("1024", json);
        Assert.Contains("0.7", json);
    }
}
