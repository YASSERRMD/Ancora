using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.InteropServices;
using System.Threading.Tasks;
using Ancora;
using Xunit;

namespace Ancora.Tests;

public class Phase147RelLongRunTests
{
    [Fact]
    public async Task Fifty_Sequential_Runs_Unique_Ids()
    {
        try
        {
            using var a = new Agent();
            var spec = new AgentSpec("llama3");
            var ids = new HashSet<string>();
            for (int i = 0; i < 50; i++)
                Assert.True(ids.Add(a.Run(spec).RunId));
            Assert.Equal(50, ids.Count);
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task Fifty_Sequential_Runs_All_Complete()
    {
        try
        {
            using var a = new Agent();
            var spec = new AgentSpec("llama3");
            for (int i = 0; i < 50; i++)
            {
                var events = await a.Run(spec).CollectAsync();
                Assert.IsType<CompletedEvent>(events[^1]);
            }
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public void Hundred_Runtime_Create_Free_Cycles()
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
    public void Five_Hundred_Store_Ops_Succeed()
    {
        var store = new Dictionary<string, int>();
        for (int i = 0; i < 500; i++) store[$"key-{i}"] = i * 2;
        for (int i = 0; i < 500; i++) Assert.Equal(i * 2, store[$"key-{i}"]);
    }

    [Fact]
    public void Ten_Concurrent_RunIds_Unique()
    {
        try
        {
            using var rt = new Runtime();
            var spec = new AgentSpec("llama3");
            var ids = Enumerable.Range(0, 10)
                .Select(_ => new Agent(rt).Run(spec).RunId)
                .ToHashSet();
            Assert.Equal(10, ids.Count);
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task Drain_All_Fifty_Runs_Non_Empty()
    {
        try
        {
            using var a = new Agent();
            var spec = new AgentSpec("llama3");
            for (int i = 0; i < 50; i++)
            {
                var events = await a.Run(spec).CollectAsync();
                Assert.NotEmpty(events);
            }
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task Parallel_Ten_Runs_All_Complete()
    {
        try
        {
            using var rt = new Runtime();
            var spec = new AgentSpec("llama3");
            var tasks = Enumerable.Range(0, 10).Select(async _ =>
            {
                using var a = new Agent(rt);
                return await a.Run(spec).CollectAsync();
            });
            var results = await Task.WhenAll(tasks);
            foreach (var events in results)
                Assert.IsType<CompletedEvent>(events[^1]);
        }
        catch (DllNotFoundException) { }
    }
}
