using System;
using System.Net;
using System.Text;
using System.Text.Json;
using System.Threading.Tasks;
using Ancora;
using Xunit;

namespace Ancora.Tests;

/// <summary>
/// Exercises the SDK surface added to close Attestra-readiness gaps: the
/// native library resolver, provider config wired to a real runtime, typed
/// cost, typed structured output, and the "failed" run event the FFI layer
/// now actually sends. Unlike many tests elsewhere in this suite, these do
/// not catch <see cref="DllNotFoundException"/> defensively: CI builds the
/// native library before running tests, so a failure here is a real
/// regression, not an expected offline gap.
/// </summary>
public class Phase148SdkSurfaceTests
{
    // ---- native library resolver -----------------------------------

    [Fact]
    public void NativeLibraryResolver_Loads_Library_Without_Env_Vars()
    {
        // If the resolver didn't work, this would throw DllNotFoundException
        // (the test runner does not set LD_LIBRARY_PATH/DYLD_LIBRARY_PATH).
        var version = Runtime.Version();
        Assert.False(string.IsNullOrEmpty(version));
    }

    // ---- offline echo run: proves real execution, not a stub --------

    [Fact]
    public async Task Default_Runtime_Echoes_Instructions_As_Output()
    {
        using var agent = new Agent();
        var handle = agent.Run(new AgentSpec("mock", Instructions: "echo this exact phrase"));
        var events = await handle.CollectAsync();

        var completed = Assert.Single(events, e => e is CompletedEvent);
        Assert.Equal("echo this exact phrase", ((CompletedEvent)completed).Output);
    }

    [Fact]
    public async Task GetCostTyped_Returns_Zero_For_Offline_Run()
    {
        using var agent = new Agent();
        var handle = agent.Run(new AgentSpec("mock", Instructions: "hi"));
        await handle.CollectAsync();

        var cost = handle.GetCostTyped();
        Assert.Equal(handle.RunId, cost.RunId);
        Assert.Equal(0.0, cost.TotalUsd);
    }

    // ---- structured output -------------------------------------------

    public record Verdict(string Decision, double Confidence);

    [Fact]
    public async Task CompletedEvent_Deserializes_Structured_Output()
    {
        var json = JsonSerializer.Serialize(new Verdict("approved", 0.92));
        using var agent = new Agent();
        var handle = agent.Run(new AgentSpec("mock", Instructions: json));
        var events = await handle.CollectAsync();

        var completed = (CompletedEvent)Assert.Single(events, e => e is CompletedEvent);
        var verdict = completed.Deserialize<Verdict>();
        Assert.NotNull(verdict);
        Assert.Equal("approved", verdict!.Decision);
        Assert.Equal(0.92, verdict.Confidence);
    }

    // ---- FailedEvent: this is a regression test ----------------------
    //
    // The FFI's real run engine can now emit {"kind":"failed",...}. Before
    // this fix, RunEventJsonConverter had no "failed" case and threw
    // JsonException the first time a real run actually failed.

    [Fact]
    public async Task Unreachable_Provider_Produces_FailedEvent_Not_An_Exception()
    {
        using var runtime = new Runtime(new ProviderConfig(
            BaseUrl: "http://127.0.0.1:1",
            AuthEnvVar: "UNSET_TEST_KEY_148"));
        using var agent = new Agent(runtime);
        var handle = agent.Run(new AgentSpec("mock", Instructions: "hello"));
        var events = await handle.CollectAsync();

        var failed = Assert.Single(events, e => e is FailedEvent);
        Assert.False(string.IsNullOrEmpty(((FailedEvent)failed).Error));
    }

    [Fact]
    public void FailedEvent_Round_Trips_Through_Json_Converter()
    {
        var original = new FailedEvent("run-1", "boom");
        var json = JsonSerializer.Serialize<RunEvent>(original, Wire148TestOptions);
        var parsed = JsonSerializer.Deserialize<RunEvent>(json, Wire148TestOptions);

        var failed = Assert.IsType<FailedEvent>(parsed);
        Assert.Equal("run-1", failed.RunId);
        Assert.Equal("boom", failed.Error);
    }

    private static readonly JsonSerializerOptions Wire148TestOptions = new()
    {
        PropertyNamingPolicy = JsonNamingPolicy.SnakeCaseLower,
        Converters = { new RunEventJsonConverter() },
    };

    // ---- ProviderConfig wire shape -------------------------------------

    [Fact]
    public void ProviderConfig_Encodes_To_Expected_Wire_Shape()
    {
        var provider = new ProviderConfig(
            BaseUrl: "https://integrate.api.nvidia.com/v1",
            AuthEnvVar: "NVIDIA_API_KEY",
            ChatCompletionsPath: "/chat/completions");
        var bytes = Wire.EncodeRuntimeConfig(provider);
        var json = JsonDocument.Parse(bytes);

        var providerElement = json.RootElement.GetProperty("provider");
        Assert.Equal(
            "https://integrate.api.nvidia.com/v1",
            providerElement.GetProperty("base_url").GetString());
        Assert.Equal("NVIDIA_API_KEY", providerElement.GetProperty("auth_env_var").GetString());
        Assert.Equal(
            "/chat/completions", providerElement.GetProperty("chat_completions_path").GetString());
    }

    [Fact]
    public void ProviderConfig_Omits_Null_Fields()
    {
        var provider = new ProviderConfig(BaseUrl: "http://localhost:8000/v1");
        var bytes = Wire.EncodeRuntimeConfig(provider);
        var json = JsonDocument.Parse(bytes);

        var providerElement = json.RootElement.GetProperty("provider");
        Assert.False(providerElement.TryGetProperty("auth_env_var", out _));
        Assert.False(providerElement.TryGetProperty("chat_completions_path", out _));
    }

    // ---- end-to-end against a real local mock HTTP server --------------
    //
    // Runs the whole stack for real: .NET -> C ABI -> ancora-core Agent ->
    // ancora-inference's HTTP client -> a real (local, offline) HTTP
    // response -> parsed back through the same chain into a CompletedEvent.

    [Fact]
    public async Task Provider_Backed_Run_Gets_Real_Response_From_Local_Server()
    {
        using var listener = new HttpListener();
        var port = GetFreePort();
        var prefix = $"http://127.0.0.1:{port}/";
        listener.Prefixes.Add(prefix);
        listener.Start();

        var serverTask = Task.Run(() =>
        {
            var ctx = listener.GetContext();
            var responseBody = """
                {"choices":[{"message":{"role":"assistant","content":"Hello from local server"}}],"usage":{"prompt_tokens":3,"completion_tokens":4}}
                """;
            var buffer = Encoding.UTF8.GetBytes(responseBody);
            ctx.Response.ContentType = "application/json";
            ctx.Response.ContentLength64 = buffer.Length;
            ctx.Response.OutputStream.Write(buffer, 0, buffer.Length);
            ctx.Response.OutputStream.Close();
        });

        try
        {
            using var runtime = new Runtime(new ProviderConfig(BaseUrl: prefix.TrimEnd('/')));
            using var agent = new Agent(runtime);
            var handle = agent.Run(new AgentSpec("mock", Instructions: "hello"));
            var events = await handle.CollectAsync();

            await serverTask.WaitAsync(TimeSpan.FromSeconds(5));

            var completed = (CompletedEvent)Assert.Single(events, e => e is CompletedEvent);
            Assert.Equal("Hello from local server", completed.Output);
        }
        finally
        {
            listener.Stop();
        }
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
