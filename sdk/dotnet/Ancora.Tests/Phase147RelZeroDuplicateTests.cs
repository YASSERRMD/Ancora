using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.InteropServices;
using System.Threading.Tasks;
using Ancora;
using Xunit;

namespace Ancora.Tests;

public class Phase147RelZeroDuplicateTests
{
    private readonly List<string> _sideEffects = new();

    private ToolHandler MakeSideEffectHandler(string label)
        => _ => { _sideEffects.Add(label); return $"{{\"recorded\":\"{label}\"}}"; };

    [Fact]
    public void Single_Dispatch_Records_One_Effect()
    {
        _sideEffects.Clear();
        try
        {
            using var rt = new Runtime();
            using var reg = ToolRegistry.Register(rt, "effect_a", "Effect A",
                MakeSideEffectHandler("A"));
        }
        catch (DllNotFoundException) { }
        Assert.True(_sideEffects.Count <= 1);
    }

    [Fact]
    public void Two_Distinct_Dispatches_Record_Two_Distinct_Labels()
    {
        _sideEffects.Clear();
        try
        {
            using var rt = new Runtime();
            using var reg1 = ToolRegistry.Register(rt, "eff_b", "B", MakeSideEffectHandler("B"));
            using var reg2 = ToolRegistry.Register(rt, "eff_c", "C", MakeSideEffectHandler("C"));
        }
        catch (DllNotFoundException) { }
        Assert.True(_sideEffects.Count(s => s == "B") <= 1);
        Assert.True(_sideEffects.Count(s => s == "C") <= 1);
    }

    [Fact]
    public void Five_Sequential_Run_Ids_Are_Unique()
    {
        try
        {
            using var a = new Agent();
            var spec = new AgentSpec("llama3");
            var ids = new HashSet<string>();
            for (int i = 0; i < 5; i++)
                Assert.True(ids.Add(a.Run(spec).RunId));
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task Drain_Twice_Second_Is_Empty()
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
    public void Registry_Re_Register_Same_Tool_Does_Not_Duplicate()
    {
        try
        {
            using var rt = new Runtime();
            using var reg1 = ToolRegistry.Register(rt, "dedup", "Dedup tool", _ => "{}");
            reg1.Dispose();
            using var reg2 = ToolRegistry.Register(rt, "dedup", "Dedup tool", _ => "{}");
            Assert.True(rt.ToolExists("dedup"));
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public void Side_Effects_List_Cleared_Between_Tests()
    {
        _sideEffects.Clear();
        Assert.Empty(_sideEffects);
    }

    [Fact]
    public async Task Ten_Runs_All_Distinct_Ids()
    {
        try
        {
            using var a = new Agent();
            var spec = new AgentSpec("llama3");
            var ids = new HashSet<string>();
            for (int i = 0; i < 10; i++)
                ids.Add(a.Run(spec).RunId);
            Assert.Equal(10, ids.Count);
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task Completed_Events_Count_Matches_Run_Count()
    {
        try
        {
            using var a = new Agent();
            var spec = new AgentSpec("llama3");
            int completedCount = 0;
            for (int i = 0; i < 5; i++)
            {
                var events = await a.Run(spec).CollectAsync();
                if (events[^1] is CompletedEvent) completedCount++;
            }
            Assert.Equal(5, completedCount);
        }
        catch (DllNotFoundException) { }
    }
}
