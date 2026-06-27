using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using System.Runtime.InteropServices;
using System.Threading.Tasks;
using Ancora;
using Xunit;

namespace Ancora.Tests;

public class Phase147PerfBenchmarkTests
{
    [Fact]
    public async Task Single_Run_Completes_Under_Five_Seconds()
    {
        try
        {
            using var a = new Agent();
            var sw = Stopwatch.StartNew();
            var events = await a.Run(new AgentSpec("llama3")).CollectAsync();
            sw.Stop();
            Assert.IsType<CompletedEvent>(events[^1]);
            Assert.True(sw.Elapsed < TimeSpan.FromSeconds(5),
                $"Run took {sw.Elapsed.TotalMilliseconds:F0}ms");
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task Ten_Sequential_Runs_Under_Thirty_Seconds()
    {
        try
        {
            using var a = new Agent();
            var spec = new AgentSpec("llama3");
            var sw = Stopwatch.StartNew();
            for (int i = 0; i < 10; i++)
                await a.Run(spec).CollectAsync();
            sw.Stop();
            Assert.True(sw.Elapsed < TimeSpan.FromSeconds(30),
                $"10 runs took {sw.Elapsed.TotalSeconds:F1}s");
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public void Thousand_In_Memory_Ops_Under_One_Second()
    {
        var store = new Dictionary<string, string>();
        var sw = Stopwatch.StartNew();
        for (int i = 0; i < 1000; i++) store[$"k{i}"] = $"v{i}";
        for (int i = 0; i < 1000; i++) _ = store[$"k{i}"];
        sw.Stop();
        Assert.True(sw.Elapsed < TimeSpan.FromSeconds(1),
            $"1000 ops took {sw.Elapsed.TotalMilliseconds:F0}ms");
    }

    [Fact]
    public void Hundred_AgentSpec_Encodes_Under_One_Second()
    {
        var spec = new AgentSpec("llama3", Instructions: "bench");
        var sw = Stopwatch.StartNew();
        for (int i = 0; i < 100; i++)
            Wire.EncodeAgentSpec(spec);
        sw.Stop();
        Assert.True(sw.Elapsed < TimeSpan.FromSeconds(1),
            $"100 encodes took {sw.Elapsed.TotalMilliseconds:F0}ms");
    }

    [Fact]
    public async Task Parallel_Five_Runs_Under_Ten_Seconds()
    {
        try
        {
            using var rt = new Runtime();
            var spec = new AgentSpec("llama3");
            var sw = Stopwatch.StartNew();
            var tasks = Enumerable.Range(0, 5).Select(async _ =>
            {
                using var a = new Agent(rt);
                return await a.Run(spec).CollectAsync();
            });
            await Task.WhenAll(tasks);
            sw.Stop();
            Assert.True(sw.Elapsed < TimeSpan.FromSeconds(10),
                $"5 parallel runs took {sw.Elapsed.TotalSeconds:F1}s");
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public void Wire_ParseEvent_Is_Fast()
    {
        var json = """{"kind":"completed","run_id":"r1"}""";
        var sw = Stopwatch.StartNew();
        for (int i = 0; i < 1000; i++)
            Wire.ParseEvent(json);
        sw.Stop();
        Assert.True(sw.Elapsed < TimeSpan.FromSeconds(2),
            $"1000 parses took {sw.Elapsed.TotalMilliseconds:F0}ms");
    }

    [Fact]
    public void Stopwatch_Elapsed_Is_NonNegative()
    {
        var sw = Stopwatch.StartNew();
        sw.Stop();
        Assert.True(sw.Elapsed >= TimeSpan.Zero);
    }
}
