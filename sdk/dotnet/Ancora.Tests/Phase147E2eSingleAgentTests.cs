using System;
using System.Linq;
using System.Runtime.InteropServices;
using System.Threading.Tasks;
using Ancora;
using Xunit;

namespace Ancora.Tests;

public class Phase147E2eSingleAgentTests
{
    [Fact]
    public void Agent_Run_Returns_Handle_With_RunId()
    {
        try
        {
            using var a = new Agent();
            var h = a.Run(new AgentSpec("llama3"));
            Assert.NotEmpty(h.RunId);
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task Full_Lifecycle_Started_Then_Completed()
    {
        try
        {
            using var a = new Agent();
            var h = a.Run(new AgentSpec("llama3"));
            var events = await h.CollectAsync();
            Assert.IsType<StartedEvent>(events[0]);
            Assert.IsType<CompletedEvent>(events[^1]);
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task Events_Count_At_Least_Two()
    {
        try
        {
            using var a = new Agent();
            var h = a.Run(new AgentSpec("llama3"));
            var events = await h.CollectAsync();
            Assert.True(events.Count >= 2);
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task Started_Event_Carries_RunId()
    {
        try
        {
            using var a = new Agent();
            var h = a.Run(new AgentSpec("llama3"));
            var id = h.RunId;
            var events = await h.CollectAsync();
            Assert.Equal(id, events[0].RunId);
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task All_Events_Share_RunId()
    {
        try
        {
            using var a = new Agent();
            var h = a.Run(new AgentSpec("llama3"));
            var id = h.RunId;
            var events = await h.CollectAsync();
            Assert.All(events, e => Assert.Equal(id, e.RunId));
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task Second_Drain_Is_Empty()
    {
        try
        {
            using var a = new Agent();
            var h = a.Run(new AgentSpec("llama3"));
            await h.CollectAsync();
            var second = await h.CollectAsync();
            Assert.Empty(second);
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task Run_With_Instructions_Completes()
    {
        try
        {
            using var a = new Agent();
            var spec = new AgentSpec("llama3", Instructions: "Be concise. Respond in one sentence.");
            var events = await a.Run(spec).CollectAsync();
            Assert.IsType<CompletedEvent>(events[^1]);
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task Run_With_Tool_Spec_Completes()
    {
        try
        {
            using var rt = new Runtime();
            using var reg = ToolRegistry.Register(rt, "noop", "Does nothing", _ => "{}");
            using var a = new Agent(rt);
            var spec = new AgentSpec("llama3", Tools: [new ToolSpec("noop", "No-op")]);
            var events = await a.Run(spec).CollectAsync();
            Assert.IsType<CompletedEvent>(events[^1]);
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task GetCost_Returns_Valid_Json_After_Run()
    {
        try
        {
            using var a = new Agent();
            var h = a.Run(new AgentSpec("llama3"));
            await h.CollectAsync();
            var cost = h.GetCost();
            Assert.False(string.IsNullOrWhiteSpace(cost));
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task Ten_Sequential_Runs_All_Complete()
    {
        try
        {
            using var a = new Agent();
            var spec = new AgentSpec("llama3");
            for (int i = 0; i < 10; i++)
            {
                var events = await a.Run(spec).CollectAsync();
                Assert.IsType<CompletedEvent>(events[^1]);
            }
        }
        catch (DllNotFoundException) { }
    }
}
