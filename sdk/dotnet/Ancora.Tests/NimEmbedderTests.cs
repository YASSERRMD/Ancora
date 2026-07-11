using System;
using System.Net;
using System.Text;
using System.Threading.Tasks;
using Ancora;
using Xunit;

namespace Ancora.Tests;

/// <summary>
/// Exercises NimEmbedder, added to close a gap in the original prompt pack
/// (Phase 13's "Embedder client via NIM" was never delivered -- the .NET SDK
/// had zero embedding capability). Runs against a real local HTTP mock
/// server, not a stub, so a broken request/response shape is a real
/// regression.
/// </summary>
public class NimEmbedderTests
{
    [Fact]
    public async Task EmbedAsync_Parses_Single_Embedding_From_Response()
    {
        using var listener = new HttpListener();
        var port = GetFreePort();
        var prefix = $"http://127.0.0.1:{port}/";
        listener.Prefixes.Add(prefix);
        listener.Start();

        var serverTask = Task.Run(() =>
        {
            var ctx = listener.GetContext();
            var body = """{"data":[{"embedding":[0.1,0.2,0.3],"index":0}]}""";
            var buffer = Encoding.UTF8.GetBytes(body);
            ctx.Response.ContentType = "application/json";
            ctx.Response.ContentLength64 = buffer.Length;
            ctx.Response.OutputStream.Write(buffer, 0, buffer.Length);
            ctx.Response.OutputStream.Close();
        });

        try
        {
            using var embedder = new NimEmbedder(
                new EmbedderConfig(BaseUrl: prefix.TrimEnd('/'), Model: "nvidia/nv-embedqa-e5-v5"));
            var vector = await embedder.EmbedAsync("hello world");

            await serverTask.WaitAsync(TimeSpan.FromSeconds(5));

            Assert.Equal(new float[] { 0.1f, 0.2f, 0.3f }, vector);
        }
        finally
        {
            listener.Stop();
        }
    }

    [Fact]
    public async Task EmbedBatchAsync_Preserves_Input_Order_Regardless_Of_Response_Order()
    {
        using var listener = new HttpListener();
        var port = GetFreePort();
        var prefix = $"http://127.0.0.1:{port}/";
        listener.Prefixes.Add(prefix);
        listener.Start();

        var serverTask = Task.Run(() =>
        {
            var ctx = listener.GetContext();
            // Response lists index 1 before index 0 -- the client must sort by index.
            var body = """{"data":[{"embedding":[1.0,0.0],"index":1},{"embedding":[0.0,1.0],"index":0}]}""";
            var buffer = Encoding.UTF8.GetBytes(body);
            ctx.Response.ContentType = "application/json";
            ctx.Response.ContentLength64 = buffer.Length;
            ctx.Response.OutputStream.Write(buffer, 0, buffer.Length);
            ctx.Response.OutputStream.Close();
        });

        try
        {
            using var embedder = new NimEmbedder(
                new EmbedderConfig(BaseUrl: prefix.TrimEnd('/'), Model: "nvidia/nv-embedqa-e5-v5"));
            var vectors = await embedder.EmbedBatchAsync(new[] { "first", "second" });

            await serverTask.WaitAsync(TimeSpan.FromSeconds(5));

            Assert.Equal(new float[] { 0.0f, 1.0f }, vectors[0]);
            Assert.Equal(new float[] { 1.0f, 0.0f }, vectors[1]);
        }
        finally
        {
            listener.Stop();
        }
    }

    [Fact]
    public async Task EmbedAsync_Sends_Bearer_Token_From_Env_Var()
    {
        using var listener = new HttpListener();
        var port = GetFreePort();
        var prefix = $"http://127.0.0.1:{port}/";
        listener.Prefixes.Add(prefix);
        listener.Start();

        Environment.SetEnvironmentVariable("TEST_NIM_KEY_EMBED", "nvapi-test-123");
        string? sawAuthHeader = null;
        var serverTask = Task.Run(() =>
        {
            var ctx = listener.GetContext();
            sawAuthHeader = ctx.Request.Headers["Authorization"];
            var body = """{"data":[{"embedding":[0.5],"index":0}]}""";
            var buffer = Encoding.UTF8.GetBytes(body);
            ctx.Response.ContentType = "application/json";
            ctx.Response.ContentLength64 = buffer.Length;
            ctx.Response.OutputStream.Write(buffer, 0, buffer.Length);
            ctx.Response.OutputStream.Close();
        });

        try
        {
            using var embedder = new NimEmbedder(new EmbedderConfig(
                BaseUrl: prefix.TrimEnd('/'),
                Model: "nvidia/nv-embedqa-e5-v5",
                AuthEnvVar: "TEST_NIM_KEY_EMBED"));
            await embedder.EmbedAsync("hi");

            await serverTask.WaitAsync(TimeSpan.FromSeconds(5));

            Assert.Equal("Bearer nvapi-test-123", sawAuthHeader);
        }
        finally
        {
            listener.Stop();
            Environment.SetEnvironmentVariable("TEST_NIM_KEY_EMBED", null);
        }
    }

    [Fact]
    public async Task EmbedAsync_Http_Error_Throws_AncorException()
    {
        using var listener = new HttpListener();
        var port = GetFreePort();
        var prefix = $"http://127.0.0.1:{port}/";
        listener.Prefixes.Add(prefix);
        listener.Start();

        var serverTask = Task.Run(() =>
        {
            var ctx = listener.GetContext();
            var body = """{"error":"rate limited"}""";
            var buffer = Encoding.UTF8.GetBytes(body);
            ctx.Response.StatusCode = 429;
            ctx.Response.ContentType = "application/json";
            ctx.Response.ContentLength64 = buffer.Length;
            ctx.Response.OutputStream.Write(buffer, 0, buffer.Length);
            ctx.Response.OutputStream.Close();
        });

        try
        {
            using var embedder = new NimEmbedder(
                new EmbedderConfig(BaseUrl: prefix.TrimEnd('/'), Model: "nvidia/nv-embedqa-e5-v5"));
            await Assert.ThrowsAsync<AncorException>(() => embedder.EmbedAsync("hi"));
            await serverTask.WaitAsync(TimeSpan.FromSeconds(5));
        }
        finally
        {
            listener.Stop();
        }
    }

    [Fact]
    public async Task EmbedBatchAsync_Empty_Input_Returns_Empty_Without_A_Request()
    {
        using var embedder = new NimEmbedder(
            new EmbedderConfig(BaseUrl: "http://127.0.0.1:1", Model: "nvidia/nv-embedqa-e5-v5"));
        var vectors = await embedder.EmbedBatchAsync(Array.Empty<string>());
        Assert.Empty(vectors);
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
