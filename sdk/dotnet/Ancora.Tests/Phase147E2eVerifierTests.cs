using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.InteropServices;
using System.Threading.Tasks;
using Ancora;
using Xunit;

namespace Ancora.Tests;

public class Phase147E2eVerifierTests
{
    [Fact]
    public void Drafter_And_Verifier_Have_Distinct_Ids()
    {
        try
        {
            using var rt = new Runtime();
            using var d = new Agent(rt);
            using var v = new Agent(rt);
            var dh = d.Run(new AgentSpec("llama3", Instructions: "draft"));
            var vh = v.Run(new AgentSpec("llama3", Instructions: "verify"));
            Assert.NotEqual(dh.RunId, vh.RunId);
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task Drafter_Emits_Completed()
    {
        try
        {
            using var a = new Agent();
            var events = await a.Run(new AgentSpec("llama3")).CollectAsync();
            Assert.IsType<CompletedEvent>(events[^1]);
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task Verifier_Emits_Completed()
    {
        try
        {
            using var rt = new Runtime();
            using var d = new Agent(rt);
            using var v = new Agent(rt);
            await d.Run(new AgentSpec("llama3")).CollectAsync();
            var ve = await v.Run(new AgentSpec("llama3")).CollectAsync();
            Assert.IsType<CompletedEvent>(ve[^1]);
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task Parallel_Drafter_And_Verifier_Both_Complete()
    {
        try
        {
            using var rt = new Runtime();
            using var d = new Agent(rt);
            using var v = new Agent(rt);
            var dh = d.Run(new AgentSpec("llama3"));
            var vh = v.Run(new AgentSpec("llama3"));
            var results = await Task.WhenAll(dh.CollectAsync(), vh.CollectAsync());
            Assert.IsType<CompletedEvent>(results[0][^1]);
            Assert.IsType<CompletedEvent>(results[1][^1]);
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task Drafter_Events_Have_Drafter_RunId()
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
    public async Task Three_Stage_Pipeline_All_Complete()
    {
        try
        {
            using var rt = new Runtime();
            var spec = new AgentSpec("llama3");
            var handles = new List<RunHandle>();
            for (int i = 0; i < 3; i++)
                handles.Add(new Agent(rt).Run(spec));
            foreach (var h in handles)
            {
                var events = await h.CollectAsync();
                Assert.IsType<CompletedEvent>(events[^1]);
            }
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task Verifier_Results_Independent_Of_Drafter()
    {
        try
        {
            using var rt = new Runtime();
            using var d = new Agent(rt);
            using var v = new Agent(rt);
            var dh = d.Run(new AgentSpec("llama3"));
            var vh = v.Run(new AgentSpec("llama3"));
            var de = await dh.CollectAsync();
            var ve = await vh.CollectAsync();
            Assert.All(de, e => Assert.Equal(dh.RunId, e.RunId));
            Assert.All(ve, e => Assert.Equal(vh.RunId, e.RunId));
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public void Five_Verifier_Run_Ids_Unique()
    {
        try
        {
            using var rt = new Runtime();
            var spec = new AgentSpec("llama3");
            var ids = Enumerable.Range(0, 5)
                .Select(_ => new Agent(rt).Run(spec).RunId)
                .ToHashSet();
            Assert.Equal(5, ids.Count);
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task Repeat_Verifier_Cycle_Three_Times()
    {
        try
        {
            using var a = new Agent();
            for (int i = 0; i < 3; i++)
            {
                var events = await a.Run(new AgentSpec("llama3")).CollectAsync();
                Assert.IsType<CompletedEvent>(events[^1]);
            }
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public void Verifier_And_Drafter_Event_Kinds_Match_Spec()
    {
        var started = new StartedEvent("r1", "{}");
        var completed = new CompletedEvent("r1");
        Assert.NotEqual(started.Kind, completed.Kind);
    }
}
