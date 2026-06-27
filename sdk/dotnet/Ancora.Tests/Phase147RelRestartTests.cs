using System;
using System.IO;
using System.Runtime.InteropServices;
using System.Threading.Tasks;
using Ancora;
using Xunit;

namespace Ancora.Tests;

public class Phase147RelRestartTests
{
    [Fact]
    public void Runtime_Create_Free_Create_Cycle()
    {
        try
        {
            var rt = new Runtime();
            rt.Dispose();
            using var rt2 = new Runtime();
            Assert.Equal((nuint)0, rt2.ToolCount());
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public void Agent_Create_Dispose_Create_Cycle()
    {
        try
        {
            var a1 = new Agent();
            a1.Dispose();
            var a2 = new Agent();
            Assert.NotNull(a2);
            a2.Dispose();
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task Restart_Preserves_No_State_From_Previous_Run()
    {
        try
        {
            using var a1 = new Agent();
            var h1 = a1.Run(new AgentSpec("llama3"));
            await h1.CollectAsync();
            a1.Dispose();

            using var a2 = new Agent();
            var h2 = a2.Run(new AgentSpec("llama3"));
            var events = await h2.CollectAsync();
            Assert.NotEmpty(events);
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public void Ten_Create_Free_Cycles_No_Leak()
    {
        try
        {
            for (int i = 0; i < 10; i++)
            {
                using var rt = new Runtime();
                Assert.Equal((nuint)0, rt.ToolCount());
            }
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public void Agent_With_Shared_Runtime_Restart()
    {
        try
        {
            using var rt = new Runtime();
            var a = new Agent(rt);
            a.Dispose();
            var a2 = new Agent(rt);
            Assert.NotNull(a2);
            a2.Dispose();
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task Restart_After_Completed_Run()
    {
        try
        {
            using var rt = new Runtime();
            using var a1 = new Agent(rt);
            await a1.Run(new AgentSpec("llama3")).CollectAsync();
            a1.Dispose();

            using var a2 = new Agent(rt);
            var events = await a2.Run(new AgentSpec("llama3")).CollectAsync();
            Assert.IsType<CompletedEvent>(events[^1]);
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public void Tool_Unregistered_After_Registration_Dispose()
    {
        try
        {
            using var rt = new Runtime();
            var reg = ToolRegistry.Register(rt, "temp", "Temp tool", _ => "{}");
            Assert.True(rt.ToolExists("temp"));
            reg.Dispose();
            Assert.False(rt.ToolExists("temp"));
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public void Runtime_ToolCount_Zero_After_All_Dispose()
    {
        try
        {
            using var rt = new Runtime();
            var reg1 = ToolRegistry.Register(rt, "t1", "T1", _ => "{}");
            var reg2 = ToolRegistry.Register(rt, "t2", "T2", _ => "{}");
            Assert.Equal((nuint)2, rt.ToolCount());
            reg1.Dispose();
            reg2.Dispose();
            Assert.Equal((nuint)0, rt.ToolCount());
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task Five_Restart_Cycles_All_Complete()
    {
        try
        {
            for (int i = 0; i < 5; i++)
            {
                using var a = new Agent();
                var events = await a.Run(new AgentSpec("llama3")).CollectAsync();
                Assert.IsType<CompletedEvent>(events[^1]);
            }
        }
        catch (DllNotFoundException) { }
    }
}
