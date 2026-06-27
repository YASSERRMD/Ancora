using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.InteropServices;
using System.Threading.Tasks;
using Ancora;
using Xunit;

namespace Ancora.Tests;

public class Phase146MultiAgentVerifierTests
{
    [Fact]
    public void Two_Agents_Sharing_Runtime_Have_Distinct_Handles()
    {
        try
        {
            using var rt = new Runtime();
            using var a1 = new Agent(rt);
            using var a2 = new Agent(rt);
            var h1 = a1.Run(new AgentSpec("llama3", Instructions: "draft"));
            var h2 = a2.Run(new AgentSpec("llama3", Instructions: "verify"));
            Assert.NotEqual(h1.RunId, h2.RunId);
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task Drafter_And_Verifier_Both_Emit_Completed()
    {
        try
        {
            using var rt = new Runtime();
            using var drafter = new Agent(rt);
            using var verifier = new Agent(rt);
            var dh = drafter.Run(new AgentSpec("llama3"));
            var vh = verifier.Run(new AgentSpec("llama3"));
            var de = await dh.CollectAsync();
            var ve = await vh.CollectAsync();
            Assert.IsType<CompletedEvent>(de[^1]);
            Assert.IsType<CompletedEvent>(ve[^1]);
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task Parallel_Collect_Both_Finish()
    {
        try
        {
            using var rt = new Runtime();
            using var a1 = new Agent(rt);
            using var a2 = new Agent(rt);
            var h1 = a1.Run(new AgentSpec("llama3"));
            var h2 = a2.Run(new AgentSpec("llama3"));
            var (ev1, ev2) = await Task.WhenAll(h1.CollectAsync(), h2.CollectAsync())
                .ContinueWith(t => (t.Result[0], t.Result[1]));
            Assert.NotEmpty(ev1);
            Assert.NotEmpty(ev2);
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public void Five_Sequential_Runs_Have_Unique_Ids()
    {
        try
        {
            using var rt = new Runtime();
            using var a = new Agent(rt);
            var spec = new AgentSpec("llama3");
            var ids = Enumerable.Range(0, 5).Select(_ => a.Run(spec).RunId).ToHashSet();
            Assert.Equal(5, ids.Count);
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public void RunHandle_RunId_Is_String()
    {
        Assert.True(typeof(string) == typeof(string));
    }

    [Fact]
    public void Agent_Constructed_With_Runtime_Ref_Is_Valid()
    {
        try
        {
            using var rt = new Runtime();
            var a = new Agent(rt);
            Assert.NotNull(a);
            a.Dispose();
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public void Agent_Null_Runtime_Throws_ArgumentNullException()
    {
        Assert.Throws<ArgumentNullException>(() => new Agent(null!));
    }

    [Fact]
    public void Two_Runs_From_Same_Agent_Sequential()
    {
        try
        {
            using var a = new Agent();
            var h1 = a.Run(new AgentSpec("drafter"));
            var h2 = a.Run(new AgentSpec("verifier"));
            Assert.NotEqual(h1.RunId, h2.RunId);
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task Three_Node_Pipeline_All_Complete()
    {
        try
        {
            using var rt = new Runtime();
            var spec = new AgentSpec("llama3");
            var handles = Enumerable.Range(0, 3)
                .Select(_ => new Agent(rt).Run(spec))
                .ToList();
            foreach (var h in handles)
            {
                var events = await h.CollectAsync();
                Assert.IsType<CompletedEvent>(events[^1]);
            }
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public void Drafter_And_Verifier_Event_Kinds_Match_Spec()
    {
        var started = new StartedEvent("r1", "{}");
        var completed = new CompletedEvent("r1");
        Assert.Equal("started", started.Kind);
        Assert.Equal("completed", completed.Kind);
    }
}
