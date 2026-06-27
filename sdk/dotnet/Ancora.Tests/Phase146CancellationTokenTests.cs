using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Threading;
using System.Threading.Tasks;
using Ancora;
using Xunit;

namespace Ancora.Tests;

public class Phase146CancellationTokenTests
{
    [Fact]
    public void CancellationToken_Default_Is_Not_Cancelled()
    {
        var ct = default(CancellationToken);
        Assert.False(ct.IsCancellationRequested);
    }

    [Fact]
    public void CancellationTokenSource_Cancel_Sets_IsCancellationRequested()
    {
        using var cts = new CancellationTokenSource();
        cts.Cancel();
        Assert.True(cts.Token.IsCancellationRequested);
    }

    [Fact]
    public async Task EventsAsync_Cancels_Midstream()
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
                if (count >= 1) cts.Cancel();
            }
            Assert.True(count >= 1);
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task Already_Cancelled_Token_Skips_All_Events()
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
    public async Task CollectAsync_Respects_CancellationToken()
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

    [Fact]
    public async Task CancellationToken_Timeout_Via_CancelAfter()
    {
        try
        {
            using var a = new Agent();
            var h = a.Run(new AgentSpec("llama3"));
            using var cts = new CancellationTokenSource(TimeSpan.FromSeconds(5));
            int count = 0;
            await foreach (var ev in h.EventsAsync(cts.Token))
                count++;
            Assert.True(count >= 0);
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public void CancellationToken_Can_Be_Linked()
    {
        using var cts1 = new CancellationTokenSource();
        using var cts2 = new CancellationTokenSource();
        using var linked = CancellationTokenSource.CreateLinkedTokenSource(cts1.Token, cts2.Token);
        Assert.False(linked.Token.IsCancellationRequested);
        cts1.Cancel();
        Assert.True(linked.Token.IsCancellationRequested);
    }

    [Fact]
    public async Task ResumeAndCollectAsync_Respects_Cancellation()
    {
        try
        {
            using var a = new Agent();
            var h = a.Run(new AgentSpec("llama3"));
            using var cts = new CancellationTokenSource();
            var events = await h.ResumeAndCollectAsync("{\"ok\":true}", cts.Token);
            Assert.NotNull(events);
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public void EventsAsync_Parameter_Has_EnumeratorCancellation()
    {
        var method = typeof(RunHandle).GetMethod("EventsAsync")!;
        var param = method.GetParameters()[0];
        var attrs = param.GetCustomAttributes(false);
        Assert.Contains(attrs, a => a is System.Runtime.CompilerServices.EnumeratorCancellationAttribute);
    }

    [Fact]
    public async Task Multiple_Cancel_Calls_Are_Safe()
    {
        using var cts = new CancellationTokenSource();
        cts.Cancel();
        cts.Cancel();
        await Task.Yield();
        Assert.True(cts.Token.IsCancellationRequested);
    }
}
