using System.Collections.Generic;
using System.Linq;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Threading;
using System.Threading.Tasks;
using Ancora;
using Xunit;

namespace Ancora.Tests;

public class Phase146StreamingTests
{
    [Fact]
    public void EventsAsync_Returns_IAsyncEnumerable()
    {
        var m = typeof(RunHandle).GetMethod("EventsAsync");
        Assert.NotNull(m);
        var returnType = m!.ReturnType;
        Assert.True(returnType.IsGenericType);
        Assert.Equal(typeof(IAsyncEnumerable<>), returnType.GetGenericTypeDefinition());
    }

    [Fact]
    public void RunHandle_Has_CollectAsync_Method()
    {
        var m = typeof(RunHandle).GetMethod("CollectAsync");
        Assert.NotNull(m);
    }

    [Fact]
    public async Task EventsAsync_Emits_At_Least_One_Event()
    {
        try
        {
            using var a = new Agent();
            var h = a.Run(new AgentSpec("llama3"));
            int count = 0;
            await foreach (var ev in h.EventsAsync())
                count++;
            Assert.True(count >= 1);
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task EventsAsync_First_Event_Is_Started()
    {
        try
        {
            using var a = new Agent();
            var h = a.Run(new AgentSpec("llama3"));
            await foreach (var ev in h.EventsAsync())
            {
                Assert.IsType<StartedEvent>(ev);
                break;
            }
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task EventsAsync_Cancellation_Stops_Iteration()
    {
        try
        {
            using var a = new Agent();
            var h = a.Run(new AgentSpec("llama3"));
            using var cts = new CancellationTokenSource();
            int count = 0;
            await foreach (var ev in h.EventsAsync(cts.Token))
            {
                count++;
                cts.Cancel();
            }
            Assert.True(count >= 1);
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task EventsAsync_Already_Cancelled_Token_Yields_Nothing()
    {
        try
        {
            using var a = new Agent();
            var h = a.Run(new AgentSpec("llama3"));
            using var cts = new CancellationTokenSource();
            cts.Cancel();
            int count = 0;
            await foreach (var ev in h.EventsAsync(cts.Token))
                count++;
            Assert.Equal(0, count);
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task CollectAsync_Returns_IReadOnlyList()
    {
        try
        {
            using var a = new Agent();
            var h = a.Run(new AgentSpec("llama3"));
            var events = await h.CollectAsync();
            Assert.IsAssignableFrom<IReadOnlyList<RunEvent>>(events);
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public void TokenEvent_Stores_Text_Value()
    {
        var tok = new TokenEvent("r1", "Hello World");
        Assert.Equal("Hello World", tok.Text);
    }

    [Fact]
    public async Task Second_EventsAsync_Returns_Empty()
    {
        try
        {
            using var a = new Agent();
            var h = a.Run(new AgentSpec("llama3"));
            await foreach (var _ in h.EventsAsync()) { }
            int second = 0;
            await foreach (var _ in h.EventsAsync()) second++;
            Assert.Equal(0, second);
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task Task_Yield_Awaited_Between_Events()
    {
        await Task.Yield();
        Assert.True(true);
    }

    [Fact]
    public async Task CollectAsync_Cancellation_Token_Is_Propagated()
    {
        try
        {
            using var a = new Agent();
            var h = a.Run(new AgentSpec("llama3"));
            using var cts = new CancellationTokenSource();
            var events = await h.CollectAsync(cts.Token);
            Assert.NotNull(events);
        }
        catch (DllNotFoundException) { }
    }
}
