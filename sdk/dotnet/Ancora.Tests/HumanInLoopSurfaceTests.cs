using System;
using System.Net;
using System.Text;
using System.Threading.Tasks;
using Ancora;
using Xunit;

namespace Ancora.Tests;

/// <summary>
/// Exercises the real human-in-loop suspend/resume surface added to close
/// an Attestra-readiness gap: previously ancora_run_resume was a synthetic
/// no-op regardless of what was passed to it. Like
/// <c>Phase148SdkSurfaceTests</c> and <c>MemoryStoreTests</c>, these do not
/// catch <see cref="DllNotFoundException"/> defensively -- CI builds the
/// native library before running tests, so a failure here is a real
/// regression.
/// </summary>
public class HumanInLoopSurfaceTests
{
    [Fact]
    public async Task Gated_Tool_Call_Suspends_Run_Then_Resume_Completes_With_Real_Response()
    {
        using var listener = new HttpListener();
        var port = GetFreePort();
        var prefix = $"http://127.0.0.1:{port}/";
        listener.Prefixes.Add(prefix);
        listener.Start();

        var serverTask = Task.Run(() =>
        {
            // First request: the model requests the gated tool call.
            var ctx1 = listener.GetContext();
            WriteJson(ctx1, """
                {"choices":[{"message":{"role":"assistant","content":null,"tool_calls":[{"id":"call_1","type":"function","function":{"name":"get_weather","arguments":"{\"city\":\"Paris\"}"}}]},"finish_reason":"tool_calls"}],"usage":{"prompt_tokens":20,"completion_tokens":8}}
                """);

            // Second request: after resume, the model produces final text.
            var ctx2 = listener.GetContext();
            WriteJson(ctx2, """
                {"choices":[{"message":{"role":"assistant","content":"it is sunny in Paris","tool_calls":[]},"finish_reason":"stop"}],"usage":{"prompt_tokens":30,"completion_tokens":6}}
                """);
        });

        try
        {
            using var runtime = new Runtime(new ProviderConfig(BaseUrl: prefix.TrimEnd('/')));
            var invoked = false;
            using var registration = ToolRegistry.RegisterRequiringApproval(
                runtime,
                "get_weather",
                "gets the weather",
                _ =>
                {
                    invoked = true;
                    return "should never be called before approval";
                });

            using var agent = new Agent(runtime);
            var handle = agent.Run(new AgentSpec("mock", Instructions: "what is the weather"));

            // CollectAsync drains events until the queue is empty, which
            // happens right after "suspended" -- no more events are queued
            // until Resume is called.
            var beforeResume = await handle.CollectAsync();
            Assert.Contains(beforeResume, e => e is StartedEvent);
            var suspended = (SuspendedEvent)Assert.Single(beforeResume, e => e is SuspendedEvent);
            Assert.Equal("get_weather", suspended.ToolName);
            Assert.Equal("call_1", suspended.ToolCallId);
            Assert.False(invoked, "gated callback must not run before a decision is supplied");

            handle.Resume(resultJson: "\"72F and sunny\"", isError: false);
            var remaining = await handle.CollectAsync();

            var resumed = Assert.Single(remaining, e => e is ResumedEvent);
            Assert.Contains("72F and sunny", ((ResumedEvent)resumed).Decision);

            var completed = (CompletedEvent)Assert.Single(remaining, e => e is CompletedEvent);
            Assert.Equal("it is sunny in Paris", completed.Output);
            Assert.False(invoked, "gated callback is never auto-invoked, even after resume");

            await serverTask.WaitAsync(TimeSpan.FromSeconds(5));
        }
        finally
        {
            listener.Stop();
        }
    }

    [Fact]
    public async Task Resume_On_Never_Suspended_Run_Keeps_Legacy_Behavior()
    {
        using var agent = new Agent();
        var handle = agent.Run(new AgentSpec("mock", Instructions: "hello"));
        await handle.CollectAsync();

        // Nothing was ever suspended for this run -- resume must be a
        // harmless no-op (the pre-existing FFI contract), not throw.
        handle.Resume("approve");
        var afterResume = await handle.CollectAsync();
        Assert.Contains(afterResume, e => e is ResumedEvent);
    }

    private static void WriteJson(HttpListenerContext ctx, string body)
    {
        var buffer = Encoding.UTF8.GetBytes(body);
        ctx.Response.ContentType = "application/json";
        ctx.Response.ContentLength64 = buffer.Length;
        ctx.Response.OutputStream.Write(buffer, 0, buffer.Length);
        ctx.Response.OutputStream.Close();
    }

    private static int GetFreePort()
    {
        using var socket = new System.Net.Sockets.Socket(
            System.Net.Sockets.AddressFamily.InterNetwork,
            System.Net.Sockets.SocketType.Stream,
            System.Net.Sockets.ProtocolType.Tcp);
        socket.Bind(new IPEndPoint(IPAddress.Loopback, 0));
        return ((IPEndPoint)socket.LocalEndPoint!).Port;
    }
}
