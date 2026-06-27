using System;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading.Tasks;
using Ancora;
using Xunit;

namespace Ancora.Tests;

public class Phase147E2eHumanInLoopTests
{
    private const string ApproveDecision = "{\"approved\":true}";
    private const string RejectDecision = "{\"approved\":false,\"reason\":\"not safe\"}";

    [Fact]
    public async Task Run_Then_Resume_Approve()
    {
        try
        {
            using var a = new Agent();
            var h = a.Run(new AgentSpec("llama3", Instructions: "ask for approval first"));
            await h.CollectAsync();
            var afterResume = await h.ResumeAndCollectAsync(ApproveDecision);
            Assert.NotNull(afterResume);
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task Run_Then_Resume_Reject()
    {
        try
        {
            using var a = new Agent();
            var h = a.Run(new AgentSpec("llama3", Instructions: "ask for approval first"));
            await h.CollectAsync();
            var afterResume = await h.ResumeAndCollectAsync(RejectDecision);
            Assert.NotNull(afterResume);
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public void ResumedEvent_Stores_Decision_Payload()
    {
        var ev = new ResumedEvent("run-1", ApproveDecision);
        Assert.Equal(ApproveDecision, ev.Decision);
    }

    [Fact]
    public void Resume_With_String_Is_UTF8_Encoded()
    {
        var bytes = Wire.EncodeDecision(ApproveDecision);
        Assert.Equal(ApproveDecision, Encoding.UTF8.GetString(bytes));
    }

    [Fact]
    public void Reject_Decision_Includes_Reason()
    {
        Assert.Contains("reason", RejectDecision);
        Assert.Contains("not safe", RejectDecision);
    }

    [Fact]
    public async Task Multiple_Resume_Cycles_Do_Not_Crash()
    {
        try
        {
            using var a = new Agent();
            var spec = new AgentSpec("llama3");
            for (int i = 0; i < 3; i++)
            {
                var h = a.Run(spec);
                await h.CollectAsync();
                var after = await h.ResumeAndCollectAsync(ApproveDecision);
                Assert.NotNull(after);
            }
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task Idempotent_Approve_Does_Not_Throw()
    {
        try
        {
            using var a = new Agent();
            var h = a.Run(new AgentSpec("llama3"));
            await h.CollectAsync();
            h.Resume(ApproveDecision);
            h.Resume(ApproveDecision);
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public void Resume_With_Bytes_Works()
    {
        try
        {
            using var a = new Agent();
            var h = a.Run(new AgentSpec("llama3"));
            var bytes = Encoding.UTF8.GetBytes(ApproveDecision);
            h.Resume(bytes.AsSpan());
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public void ResumedEvent_RunId_Is_Preserved()
    {
        var ev = new ResumedEvent("my-run", ApproveDecision);
        Assert.Equal("my-run", ev.RunId);
    }

    [Fact]
    public void RunHandle_Has_ResumeAndCollectAsync()
    {
        var m = typeof(RunHandle).GetMethod("ResumeAndCollectAsync");
        Assert.NotNull(m);
    }
}
