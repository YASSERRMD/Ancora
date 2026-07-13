using System;
using System.Net;
using System.Text;
using System.Threading.Tasks;
using Ancora;
using Xunit;

namespace Ancora.Tests;

/// <summary>
/// Exercises AgentSpec.OutputSchemaJson, added to close a gap the original
/// prompt pack's Phase 7 explicitly asked for (validate structured output,
/// repair on failure, bound the attempts) but that a later audit found was
/// never wired into Agent::run_loop -- ancora-core's validate_with_repair
/// existed and was unit-tested in isolation, but nothing called it from the
/// real loop, so a .NET caller's malformed model output was never caught or
/// retried. Like <c>Phase148SdkSurfaceTests</c>, these do not catch
/// <see cref="DllNotFoundException"/> defensively: CI builds the native
/// library before running tests, so a failure here is a real regression.
/// </summary>
public class StructuredOutputValidationTests
{
    [Fact]
    public async Task Invalid_Output_Triggers_One_Repair_Then_Completes()
    {
        using var listener = new HttpListener();
        var port = GetFreePort();
        var prefix = $"http://127.0.0.1:{port}/";
        listener.Prefixes.Add(prefix);
        listener.Start();

        var serverTask = Task.Run(async () =>
        {
            // First response is not valid JSON -- triggers a repair request.
            await RespondChatText(listener, "not json at all");
            // Second response is valid JSON -- the repaired output.
            await RespondChatText(listener, """{"decision":"approved"}""");
        });

        try
        {
            using var runtime = new Runtime(new ProviderConfig(BaseUrl: prefix.TrimEnd('/')));
            using var agent = new Agent(runtime);
            var handle = agent.Run(new AgentSpec(
                Model: "mock",
                Instructions: "produce a decision",
                OutputSchemaJson: """{"type":"object"}"""));

            var events = await handle.CollectAsync();
            var completed = (CompletedEvent)Assert.Single(events, e => e is CompletedEvent);
            Assert.Equal("""{"decision":"approved"}""", completed.Output);

            await serverTask.WaitAsync(TimeSpan.FromSeconds(5));
        }
        finally
        {
            listener.Stop();
        }
    }

    [Fact]
    public async Task Output_That_Never_Becomes_Valid_Produces_A_FailedEvent()
    {
        using var listener = new HttpListener();
        var port = GetFreePort();
        var prefix = $"http://127.0.0.1:{port}/";
        listener.Prefixes.Add(prefix);
        listener.Start();

        var serverTask = Task.Run(async () =>
        {
            // Every response is invalid. With a 3-attempt repair budget,
            // this is exactly 1 initial call + 2 repair calls before the
            // budget runs out -- must not loop forever or silently return
            // bad output.
            for (var i = 0; i < 3; i++)
            {
                await RespondChatText(listener, "still not json");
            }
        });

        try
        {
            using var runtime = new Runtime(new ProviderConfig(BaseUrl: prefix.TrimEnd('/')));
            using var agent = new Agent(runtime);
            var handle = agent.Run(new AgentSpec(
                Model: "mock",
                Instructions: "produce a decision",
                OutputSchemaJson: """{"type":"object"}"""));

            var events = await handle.CollectAsync();
            Assert.Contains(events, e => e is FailedEvent);

            await serverTask.WaitAsync(TimeSpan.FromSeconds(5));
        }
        finally
        {
            listener.Stop();
        }
    }

    [Fact]
    public async Task Empty_Schema_Never_Validates_Or_Repairs()
    {
        // No OutputSchemaJson set (the default) -- malformed text must pass
        // through unvalidated, exactly like the SDK behaved before this
        // capability existed.
        using var agent = new Agent();
        var handle = agent.Run(new AgentSpec(Model: "mock", Instructions: "not json at all"));
        var events = await handle.CollectAsync();

        var completed = (CompletedEvent)Assert.Single(events, e => e is CompletedEvent);
        Assert.Equal("not json at all", completed.Output);
    }

    private static async Task RespondChatText(HttpListener listener, string content)
    {
        var ctx = await listener.GetContextAsync();
        var responseBody = new
        {
            choices = new[]
            {
                new
                {
                    message = new { role = "assistant", content, tool_calls = Array.Empty<object>() },
                    finish_reason = "stop",
                },
            },
            usage = new { prompt_tokens = 10, completion_tokens = 4 },
        };
        var buffer = Encoding.UTF8.GetBytes(System.Text.Json.JsonSerializer.Serialize(responseBody));
        ctx.Response.ContentType = "application/json";
        ctx.Response.ContentLength64 = buffer.Length;
        await ctx.Response.OutputStream.WriteAsync(buffer);
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
