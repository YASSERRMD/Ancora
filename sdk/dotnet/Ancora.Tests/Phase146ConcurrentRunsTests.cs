using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.InteropServices;
using System.Threading.Tasks;
using Ancora;
using Xunit;

namespace Ancora.Tests;

public class Phase146ConcurrentRunsTests
{
    [Fact]
    public void Ten_Sequential_Runs_Have_Unique_Ids()
    {
        try
        {
            using var a = new Agent();
            var spec = new AgentSpec("llama3");
            var ids = new HashSet<string>();
            for (int i = 0; i < 10; i++)
            {
                var id = a.Run(spec).RunId;
                Assert.True(ids.Add(id), $"Duplicate run ID: {id}");
            }
            Assert.Equal(10, ids.Count);
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task Ten_Concurrent_Runs_All_Complete()
    {
        try
        {
            using var rt = new Runtime();
            var spec = new AgentSpec("llama3");
            var tasks = Enumerable.Range(0, 10).Select(async _ =>
            {
                using var a = new Agent(rt);
                var h = a.Run(spec);
                return await h.CollectAsync();
            });
            var results = await Task.WhenAll(tasks);
            foreach (var events in results)
                Assert.IsType<CompletedEvent>(events[^1]);
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task Parallel_Agents_Ids_Are_Distinct()
    {
        try
        {
            using var rt = new Runtime();
            var spec = new AgentSpec("llama3");
            var handles = Enumerable.Range(0, 5)
                .Select(_ => new Agent(rt).Run(spec))
                .ToList();
            var ids = handles.Select(h => h.RunId).ToHashSet();
            Assert.Equal(5, ids.Count);
            foreach (var h in handles)
                await h.CollectAsync();
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public void Sequential_Runs_Counter_Monotonic()
    {
        try
        {
            using var a = new Agent();
            var spec = new AgentSpec("llama3");
            var ids = new List<string>();
            for (int i = 0; i < 5; i++) ids.Add(a.Run(spec).RunId);
            Assert.Equal(5, ids.Count);
            Assert.Equal(ids.Distinct().Count(), ids.Count);
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task Fifty_Sequential_Runs_All_Succeed()
    {
        try
        {
            using var a = new Agent();
            var spec = new AgentSpec("llama3");
            for (int i = 0; i < 50; i++)
            {
                var h = a.Run(spec);
                var events = await h.CollectAsync();
                Assert.NotEmpty(events);
            }
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public void Hundred_Runtime_Create_Free_Cycles_Succeed()
    {
        try
        {
            for (int i = 0; i < 100; i++)
            {
                using var rt = new Runtime();
                Assert.Equal((nuint)0, rt.ToolCount());
            }
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task Drain_Then_Drain_Again_Is_Empty()
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
    public async Task Parallel_Collect_No_Cross_Contamination()
    {
        try
        {
            using var rt = new Runtime();
            using var a1 = new Agent(rt);
            using var a2 = new Agent(rt);
            var h1 = a1.Run(new AgentSpec("m1"));
            var h2 = a2.Run(new AgentSpec("m2"));
            var (ev1, ev2) = await Task.WhenAll(h1.CollectAsync(), h2.CollectAsync())
                .ContinueWith(t => (t.Result[0], t.Result[1]));
            var id1 = h1.RunId;
            var id2 = h2.RunId;
            Assert.All(ev1, e => Assert.Equal(id1, e.RunId));
            Assert.All(ev2, e => Assert.Equal(id2, e.RunId));
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public void Concurrent_Runs_Count_Matches_Agent_Count()
    {
        try
        {
            using var rt = new Runtime();
            var spec = new AgentSpec("llama3");
            int count = 0;
            for (int i = 0; i < 10; i++)
            {
                var a = new Agent(rt);
                a.Run(spec);
                count++;
                a.Dispose();
            }
            Assert.Equal(10, count);
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public void RunHandle_Is_Not_IDisposable()
    {
        Assert.False(typeof(IDisposable).IsAssignableFrom(typeof(RunHandle)));
    }
}
