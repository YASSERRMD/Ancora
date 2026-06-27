using System;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading.Tasks;
using Ancora;
using Xunit;

namespace Ancora.Tests;

public class Phase146HumanInLoopTests
{
    private const string ApproveJson = "{\"approved\":true}";
    private const string RejectJson = "{\"approved\":false,\"reason\":\"not safe\"}";

    [Fact]
    public void RunHandle_Has_Resume_Method()
    {
        var m = typeof(RunHandle).GetMethod("Resume", [typeof(string)]);
        Assert.NotNull(m);
    }

    [Fact]
    public void RunHandle_Has_Resume_Bytes_Method()
    {
        var m = typeof(RunHandle).GetMethod("Resume", [typeof(ReadOnlySpan<byte>)]);
        Assert.NotNull(m);
    }

    [Fact]
    public void RunHandle_Has_ResumeAndCollectAsync_Method()
    {
        var m = typeof(RunHandle).GetMethod("ResumeAndCollectAsync");
        Assert.NotNull(m);
    }

    [Fact]
    public void Wire_EncodeDecision_Produces_UTF8()
    {
        var bytes = Wire.EncodeDecision(ApproveJson);
        var decoded = Encoding.UTF8.GetString(bytes);
        Assert.Equal(ApproveJson, decoded);
    }

    [Fact]
    public void Wire_EncodeDecision_Reject_Produces_UTF8()
    {
        var bytes = Wire.EncodeDecision(RejectJson);
        var decoded = Encoding.UTF8.GetString(bytes);
        Assert.Equal(RejectJson, decoded);
    }

    [Fact]
    public async Task Resume_And_Collect_After_Suspend()
    {
        try
        {
            using var a = new Agent();
            var h = a.Run(new AgentSpec("llama3", Instructions: "ask for approval"));
            await h.CollectAsync();
            var eventsAfterResume = await h.ResumeAndCollectAsync(ApproveJson);
            Assert.NotNull(eventsAfterResume);
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public void ResumedEvent_Kind_Is_Resumed()
    {
        var ev = new ResumedEvent("run-1", ApproveJson);
        Assert.Equal("resumed", ev.Kind);
        Assert.Equal(ApproveJson, ev.Decision);
    }

    [Fact]
    public void ResumedEvent_Stores_RunId()
    {
        var ev = new ResumedEvent("my-run-id", RejectJson);
        Assert.Equal("my-run-id", ev.RunId);
    }

    [Fact]
    public void RunHandle_Null_Decision_Throws()
    {
        try
        {
            using var a = new Agent();
            var h = a.Run(new AgentSpec("llama3"));
            Assert.Throws<ArgumentNullException>(() => h.Resume(null!));
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task Idempotent_Collect_After_Completed_Is_Empty()
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
}
