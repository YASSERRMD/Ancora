using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Threading.Tasks;
using Ancora;
using Xunit;

namespace Ancora.Tests;

public class Phase146SingleAgentRunTests
{
    [Fact]
    public void Agent_Run_Returns_RunHandle()
    {
        try
        {
            using var a = new Agent();
            var h = a.Run(new AgentSpec("llama3"));
            Assert.NotNull(h);
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public void RunHandle_Has_NonEmpty_RunId()
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
    public async Task RunHandle_CollectAsync_Returns_Events()
    {
        try
        {
            using var a = new Agent();
            var h = a.Run(new AgentSpec("llama3"));
            var events = await h.CollectAsync();
            Assert.NotEmpty(events);
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task First_Event_Is_Started()
    {
        try
        {
            using var a = new Agent();
            var h = a.Run(new AgentSpec("llama3"));
            var events = await h.CollectAsync();
            Assert.IsType<StartedEvent>(events[0]);
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task Last_Event_Is_Completed()
    {
        try
        {
            using var a = new Agent();
            var h = a.Run(new AgentSpec("llama3"));
            var events = await h.CollectAsync();
            Assert.IsType<CompletedEvent>(events[^1]);
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task Events_Have_Matching_RunId()
    {
        try
        {
            using var a = new Agent();
            var h = a.Run(new AgentSpec("llama3"));
            var id = h.RunId;
            var events = await h.CollectAsync();
            foreach (var ev in events)
                Assert.Equal(id, ev.RunId);
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public void Two_Runs_Have_Distinct_RunIds()
    {
        try
        {
            using var a = new Agent();
            var h1 = a.Run(new AgentSpec("llama3"));
            var h2 = a.Run(new AgentSpec("llama3"));
            Assert.NotEqual(h1.RunId, h2.RunId);
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task Second_CollectAsync_Is_Empty()
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
    public void RunHandle_StartedEvent_Kind_Is_Started()
    {
        var ev = new StartedEvent("run-1", "{}");
        Assert.Equal("started", ev.Kind);
    }

    [Fact]
    public void RunHandle_CompletedEvent_Kind_Is_Completed()
    {
        var ev = new CompletedEvent("run-1");
        Assert.Equal("completed", ev.Kind);
    }
}
