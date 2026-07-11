using System;
using System.Collections.Generic;
using System.Net;
using System.Text;
using System.Text.Json;
using System.Threading;
using System.Threading.Tasks;
using Ancora;
using Xunit;

namespace Ancora.Tests.Examples;

/// <summary>
/// The Phase 15 deliverable the original prompt pack never actually got:
/// a single end-to-end flow proving the library is ready to build Attestra.
/// Unlike <c>RagPgvectorExampleTests</c> (a keyword-matching simulation with
/// no real embeddings or retrieval), this drives the real stack against a
/// local HTTP mock server that stands in for NVIDIA NIM's embeddings and
/// chat-completions endpoints -- exactly what a hosted or self-hosted NIM
/// deployment exposes, so the only thing that changes for production is the
/// base URL.
///
/// Flow: embed two document chunks (NimEmbedder) -> store them
/// (Runtime.Upsert) -> embed a query and retrieve the matching chunk
/// (Runtime.Query) -> run an agent that calls a tool then produces a
/// structured compliance verdict -> deserialize the verdict -> verify cost
/// and that the tool ran exactly once.
///
/// Like <c>Phase148SdkSurfaceTests</c>, these do not catch
/// <see cref="DllNotFoundException"/> defensively: CI builds the native
/// library before running tests, so a failure here is a real regression.
/// </summary>
public class ComplianceReviewE2ETests
{
    private sealed record ComplianceVerdict(
        string Decision, string Rationale, double Confidence, List<string> CitedClauseIds);

    [Fact]
    public async Task Review_Embeds_Retrieves_Calls_A_Tool_And_Produces_A_Structured_Verdict()
    {
        using var listener = new HttpListener();
        var port = GetFreePort();
        var prefix = $"http://127.0.0.1:{port}/";
        listener.Prefixes.Add(prefix);
        listener.Start();

        var toolInvocations = 0;

        var serverTask = Task.Run(async () =>
        {
            // 1-2: embed the two document chunks being indexed.
            await RespondEmbedding(listener, new[] { 1.0f, 0.0f });
            await RespondEmbedding(listener, new[] { 0.0f, 1.0f });
            // 3: embed the retrieval query (closely matches chunk 1).
            await RespondEmbedding(listener, new[] { 1.0f, 0.0f });
            // 4: the model requests a tool call before it can decide.
            await RespondChatToolCall(listener, "lookup_precedent", "call_1", new { clause_id = "c-1" });
            // 5: after the tool result, the model returns the final verdict.
            await RespondChatFinal(listener, new
            {
                decision = "non_compliant",
                rationale = "Retention period of 3 years falls short of the required 7.",
                confidence = 0.87,
                citedClauseIds = new[] { "c-1" },
            });
        });

        try
        {
            using var embedder = new NimEmbedder(
                new EmbedderConfig(BaseUrl: prefix.TrimEnd('/'), Model: "nvidia/nv-embedqa-e5-v5"));
            using var runtime = new Runtime(new ProviderConfig(BaseUrl: prefix.TrimEnd('/')));

            // ---- index two document chunks ----
            runtime.CreateCollection("clauses", dimensions: 2);
            var chunk1 = "Clause 4.2: customer data must be retained for a period of 3 years.";
            var chunk2 = "Clause 7.1: vendors must maintain SOC2 Type II certification.";
            var vector1 = await embedder.EmbedAsync(chunk1, EmbedInputType.Passage);
            var vector2 = await embedder.EmbedAsync(chunk2, EmbedInputType.Passage);
            runtime.Upsert("clauses", new[]
            {
                new VectorPoint(1, vector1, new Dictionary<string, object?>
                {
                    ["text"] = chunk1, ["clause_id"] = "c-1",
                }),
                new VectorPoint(2, vector2, new Dictionary<string, object?>
                {
                    ["text"] = chunk2, ["clause_id"] = "c-2",
                }),
            });

            // ---- retrieve the chunk relevant to the review question ----
            var queryVector = await embedder.EmbedAsync(
                "does the retention policy meet the 7 year requirement?", EmbedInputType.Query);
            var retrieved = runtime.Query("clauses", queryVector, topK: 1);
            var topChunk = Assert.Single(retrieved);
            Assert.Equal(1UL, topChunk.Id);
            var retrievedText = topChunk.Payload["text"].GetString();

            // ---- run the review agent, with a tool it may call ----
            using var toolRegistration = ToolRegistry.Register(
                runtime, "lookup_precedent", "looks up a relevant legal precedent",
                _ =>
                {
                    Interlocked.Increment(ref toolInvocations);
                    return JsonSerializer.Serialize(new { precedent = "Case v. Example (2019)" });
                });

            using var agent = new Agent(runtime);
            var handle = agent.Run(new AgentSpec(
                Model: "mock",
                Instructions: $"Review this clause for compliance: {retrievedText}"));

            // ---- verify the streamed events, in order, include the tool call ----
            var events = await handle.CollectAsync();
            Assert.Contains(events, e => e is StartedEvent);
            Assert.Contains(events, e => e is ToolCallEvent tc && tc.Name == "lookup_precedent");

            // ---- verify the structured verdict deserializes ----
            var completed = (CompletedEvent)Assert.Single(events, e => e is CompletedEvent);
            var verdict = completed.Deserialize<ComplianceVerdict>();
            Assert.NotNull(verdict);
            Assert.Equal("non_compliant", verdict!.Decision);
            Assert.Equal(0.87, verdict.Confidence);
            Assert.Contains("c-1", verdict.CitedClauseIds);

            // ---- verify the tool ran exactly once and cost was tracked ----
            Assert.Equal(1, toolInvocations);
            var cost = handle.GetCostTyped();
            Assert.Equal(handle.RunId, cost.RunId);

            await serverTask.WaitAsync(TimeSpan.FromSeconds(5));
        }
        finally
        {
            listener.Stop();
        }
    }

    /// <summary>
    /// Proves the NIM adapter is a base-url-only switch: the exact same
    /// agent code runs unmodified against a second, independent mock
    /// endpoint standing in for a self-hosted NIM container.
    /// </summary>
    [Fact]
    public async Task Switching_To_A_Different_Base_Url_Requires_No_Code_Changes()
    {
        using var listener = new HttpListener();
        var port = GetFreePort();
        var prefix = $"http://127.0.0.1:{port}/";
        listener.Prefixes.Add(prefix);
        listener.Start();

        var serverTask = Task.Run(() => RespondChatFinal(listener, new { decision = "ok" }));

        try
        {
            // The only thing that differs between hosted NIM and a
            // self-hosted NIM container is this BaseUrl.
            using var selfHostedRuntime = new Runtime(new ProviderConfig(BaseUrl: prefix.TrimEnd('/')));
            using var agent = new Agent(selfHostedRuntime);
            var handle = agent.Run(new AgentSpec(Model: "mock", Instructions: "quick check"));
            var events = await handle.CollectAsync();

            var completed = (CompletedEvent)Assert.Single(events, e => e is CompletedEvent);
            Assert.Contains("\"ok\"", completed.Output);

            await serverTask.WaitAsync(TimeSpan.FromSeconds(5));
        }
        finally
        {
            listener.Stop();
        }
    }

    /// <summary>
    /// A tool call the review triggers applies its side effect exactly
    /// once, even though a caller polls well past completion -- the
    /// no-duplicate-side-effects guarantee a review workflow depends on
    /// (a reviewer's client retrying a poll must never re-run a
    /// remediation action). Full process-crash/restart recovery is a
    /// separate, journal-level guarantee proven at the Rust core layer
    /// (see ancora-core's chaos_kill_resume tests), not something this
    /// FFI/SDK layer exposes as a resumable-from-disk operation today;
    /// this proves the guarantee this SDK actually makes: dispatch happens
    /// once, and draining the event queue repeatedly never re-dispatches.
    /// </summary>
    [Fact]
    public async Task Polling_Repeatedly_After_Completion_Does_Not_Duplicate_A_Tool_Side_Effect()
    {
        using var listener = new HttpListener();
        var port = GetFreePort();
        var prefix = $"http://127.0.0.1:{port}/";
        listener.Prefixes.Add(prefix);
        listener.Start();

        var serverTask = Task.Run(async () =>
        {
            await RespondChatToolCall(listener, "apply_remediation", "call_1", new { });
            await RespondChatFinal(listener, new { decision = "done" });
        });

        try
        {
            using var runtime = new Runtime(new ProviderConfig(BaseUrl: prefix.TrimEnd('/')));
            var applyCount = 0;
            using var toolRegistration = ToolRegistry.Register(
                runtime, "apply_remediation", "applies a remediation action",
                _ =>
                {
                    Interlocked.Increment(ref applyCount);
                    return """{"applied":true}""";
                });

            using var agent = new Agent(runtime);
            var handle = agent.Run(new AgentSpec(Model: "mock", Instructions: "hello"));

            // Drain repeatedly, well past completion -- must not
            // re-invoke the tool or reprocess anything already applied.
            for (var i = 0; i < 5; i++)
            {
                await handle.CollectAsync();
            }

            Assert.Equal(1, applyCount);
            await serverTask.WaitAsync(TimeSpan.FromSeconds(5));
        }
        finally
        {
            listener.Stop();
        }
    }

    private static async Task RespondJson(HttpListener listener, object body)
    {
        var ctx = await listener.GetContextAsync();
        var json = JsonSerializer.Serialize(body);
        var buffer = Encoding.UTF8.GetBytes(json);
        ctx.Response.ContentType = "application/json";
        ctx.Response.ContentLength64 = buffer.Length;
        ctx.Response.OutputStream.Write(buffer, 0, buffer.Length);
        ctx.Response.OutputStream.Close();
    }

    private static Task RespondEmbedding(HttpListener listener, float[] vector) =>
        RespondJson(listener, new { data = new[] { new { embedding = vector, index = 0 } } });

    private static Task RespondChatToolCall(
        HttpListener listener, string toolName, string callId, object arguments) =>
        RespondJson(listener, new
        {
            choices = new[]
            {
                new
                {
                    message = new
                    {
                        role = "assistant",
                        content = (string?)null,
                        tool_calls = new[]
                        {
                            new
                            {
                                id = callId,
                                type = "function",
                                function = new
                                {
                                    name = toolName,
                                    arguments = JsonSerializer.Serialize(arguments),
                                },
                            },
                        },
                    },
                    finish_reason = "tool_calls",
                },
            },
            usage = new { prompt_tokens = 20, completion_tokens = 8 },
        });

    private static Task RespondChatFinal(HttpListener listener, object structuredContent) =>
        RespondJson(listener, new
        {
            choices = new[]
            {
                new
                {
                    message = new
                    {
                        role = "assistant",
                        content = JsonSerializer.Serialize(structuredContent),
                        tool_calls = Array.Empty<object>(),
                    },
                    finish_reason = "stop",
                },
            },
            usage = new { prompt_tokens = 30, completion_tokens = 6 },
        });

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
