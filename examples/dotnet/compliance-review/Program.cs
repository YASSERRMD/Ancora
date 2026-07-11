using System.Net;
using System.Text;
using System.Text.Json;
using Ancora;

Console.WriteLine("Ancora .NET SDK: compliance review example");
Console.WriteLine("============================================");
Console.WriteLine();
Console.WriteLine("Retrieval-augmented, structured-output compliance review against an");
Console.WriteLine("NVIDIA NIM-compatible endpoint (embeddings + chat completions).");
Console.WriteLine();

// This example runs fully offline by default, against a tiny local server
// standing in for NIM, so it works with zero setup. Point it at a real
// deployment (hosted NVIDIA NIM or a self-hosted NIM container) by setting
// these two environment variables -- switching endpoints is a base-url
// change only:
//   NIM_BASE_URL     e.g. https://integrate.api.nvidia.com/v1
//   NVIDIA_API_KEY   your nvapi-... key (hosted only; self-hosted containers
//                    are usually unauthenticated)
var baseUrl = Environment.GetEnvironmentVariable("NIM_BASE_URL");
DemoNimServer? demoServer = null;
// The demo server needs no auth (like a typical self-hosted container); a
// real hosted NIM deployment does, via a bearer token read from this env
// var. Only ask the runtime to require it when actually talking to one.
string? authEnvVar = "NVIDIA_API_KEY";
if (string.IsNullOrEmpty(baseUrl))
{
    demoServer = DemoNimServer.Start();
    baseUrl = demoServer.BaseUrl;
    authEnvVar = null;
    Console.WriteLine($"NIM_BASE_URL not set -- using an in-process demo server at {baseUrl}");
}
else
{
    Console.WriteLine($"Using NIM_BASE_URL={baseUrl}");
}
Console.WriteLine();

try
{
    var providerConfig = new ProviderConfig(BaseUrl: baseUrl, AuthEnvVar: authEnvVar);

    using var embedder = new NimEmbedder(
        new EmbedderConfig(BaseUrl: baseUrl, Model: "nvidia/nv-embedqa-e5-v5", AuthEnvVar: authEnvVar));
    using var runtime = new Runtime(providerConfig);

    // --- Step 1: index the contract clauses under review ---
    Console.WriteLine("Indexing contract clauses...");
    runtime.CreateCollection("clauses", dimensions: 2);

    var clauses = new[]
    {
        (Id: 1UL, ClauseId: "c-1", Text: "Clause 4.2: customer data must be retained for a period of 3 years."),
        (Id: 2UL, ClauseId: "c-2", Text: "Clause 7.1: vendors must maintain SOC2 Type II certification."),
    };

    foreach (var clause in clauses)
    {
        var vector = await embedder.EmbedAsync(clause.Text, EmbedInputType.Passage);
        runtime.Upsert("clauses", new[]
        {
            new VectorPoint(clause.Id, vector, new Dictionary<string, object?>
            {
                ["text"] = clause.Text,
                ["clause_id"] = clause.ClauseId,
            }),
        });
        Console.WriteLine($"  indexed {clause.ClauseId}");
    }
    Console.WriteLine();

    // --- Step 2: retrieve the clause relevant to the review question ---
    var question = "does the retention policy meet the 7 year regulatory requirement?";
    Console.WriteLine($"Review question: {question}");
    var queryVector = await embedder.EmbedAsync(question, EmbedInputType.Query);
    var retrieved = runtime.Query("clauses", queryVector, topK: 1);
    var topClause = retrieved.First();
    var retrievedText = topClause.Payload["text"].GetString();
    Console.WriteLine($"Retrieved: {retrievedText}");
    Console.WriteLine();

    // --- Step 3: register a tool the review agent can call ---
    using var toolRegistration = ToolRegistry.Register(
        runtime, "lookup_precedent", "looks up a relevant legal precedent",
        input =>
        {
            Console.WriteLine($"  [tool call] lookup_precedent({input.GetRawText()})");
            return JsonSerializer.Serialize(new { precedent = "Case v. Example (2019)" });
        });

    // --- Step 4: run the review agent ---
    using var agent = new Agent(runtime);
    var handle = agent.Run(new AgentSpec(
        Model: "meta/llama-3.1-70b-instruct",
        Instructions: $"Review this clause for compliance: {retrievedText}"));

    Console.WriteLine($"Run ID: {handle.RunId}");
    Console.WriteLine();

    CompletedEvent? completed = null;
    FailedEvent? failedEvent = null;
    await foreach (var ev in handle.EventsAsync())
    {
        switch (ev)
        {
            case StartedEvent:
                Console.WriteLine("Run started.");
                break;
            case ToolCallEvent tc:
                Console.WriteLine($"Model requested tool: {tc.Name}");
                break;
            case CompletedEvent c:
                completed = c;
                break;
            case FailedEvent failed:
                failedEvent = failed;
                Console.WriteLine($"Run failed: {failed.Error}");
                break;
        }
    }
    Console.WriteLine();

    // A demo whose run fails should exit non-zero -- otherwise a broken
    // example silently "succeeds" in CI while printing nothing useful.
    if (failedEvent is not null)
    {
        Console.Error.WriteLine($"Example failed: {failedEvent.Error}");
        Environment.Exit(1);
    }

    // --- Step 5: parse the structured verdict ---
    if (completed is null)
    {
        Console.Error.WriteLine("Example failed: run produced no completed event.");
        Environment.Exit(1);
    }

    var verdict = completed!.Deserialize<ComplianceVerdict>();
    if (verdict is not null)
    {
        Console.WriteLine("Verdict:");
        Console.WriteLine($"  decision:   {verdict.Decision}");
        Console.WriteLine($"  confidence: {verdict.Confidence:P0}");
        Console.WriteLine($"  rationale:  {verdict.Rationale}");
        Console.WriteLine($"  cited:      {string.Join(", ", verdict.CitedClauseIds)}");
    }
    else
    {
        Console.Error.WriteLine($"Example failed: could not parse structured verdict from output: {completed.Output}");
        Environment.Exit(1);
    }
    Console.WriteLine();

    var cost = handle.GetCostTyped();
    Console.WriteLine($"Cost: ${cost.TotalUsd:F6}");
}
finally
{
    demoServer?.Dispose();
}

internal sealed record ComplianceVerdict(
    string Decision, string Rationale, double Confidence, List<string> CitedClauseIds);

/// <summary>
/// A tiny in-process HTTP server standing in for NVIDIA NIM's embeddings
/// and chat-completions endpoints, so this example runs with zero setup.
/// A real deployment (hosted or self-hosted NIM) is a drop-in replacement:
/// set NIM_BASE_URL and this class is never used.
/// </summary>
internal sealed class DemoNimServer : IDisposable
{
    private readonly HttpListener _listener;
    private readonly Task _serverTask;

    public string BaseUrl { get; }

    private DemoNimServer(HttpListener listener, string baseUrl, Task serverTask)
    {
        _listener = listener;
        BaseUrl = baseUrl;
        _serverTask = serverTask;
    }

    public static DemoNimServer Start()
    {
        using var socket = new System.Net.Sockets.Socket(
            System.Net.Sockets.AddressFamily.InterNetwork,
            System.Net.Sockets.SocketType.Stream,
            System.Net.Sockets.ProtocolType.Tcp);
        socket.Bind(new IPEndPoint(IPAddress.Loopback, 0));
        var port = ((IPEndPoint)socket.LocalEndPoint!).Port;
        socket.Close();

        var listener = new HttpListener();
        var prefix = $"http://127.0.0.1:{port}/";
        listener.Prefixes.Add(prefix);
        listener.Start();

        var serverTask = Task.Run(async () =>
        {
            try
            {
                while (listener.IsListening)
                {
                    var ctx = await listener.GetContextAsync();
                    await HandleAsync(ctx);
                }
            }
            catch (HttpListenerException)
            {
                // Listener was stopped; exit quietly.
            }
            catch (ObjectDisposedException)
            {
                // Listener was disposed; exit quietly.
            }
        });

        return new DemoNimServer(listener, prefix.TrimEnd('/'), serverTask);
    }

    private static async Task HandleAsync(HttpListenerContext ctx)
    {
        object body;
        if (ctx.Request.Url!.AbsolutePath.EndsWith("/embeddings", StringComparison.Ordinal))
        {
            // A real embedder returns a semantically meaningful vector; this
            // demo stands in with a tiny deterministic rule instead (does the
            // request mention "retention"?) so retrieval genuinely picks the
            // clause relevant to the question, not just an arbitrary one.
            using var reader = new StreamReader(ctx.Request.InputStream);
            var requestBody = await reader.ReadToEndAsync();
            var mentionsRetention =
                requestBody.Contains("retain", StringComparison.OrdinalIgnoreCase) ||
                requestBody.Contains("retention", StringComparison.OrdinalIgnoreCase);
            var embedding = mentionsRetention ? new[] { 1.0f, 0.0f } : new[] { 0.0f, 1.0f };
            body = new { data = new[] { new { embedding, index = 0 } } };
        }
        else if (Interlocked.Increment(ref _chatCallCount) == 1)
        {
            body = new
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
                                    id = "call_1",
                                    type = "function",
                                    function = new
                                    {
                                        name = "lookup_precedent",
                                        arguments = JsonSerializer.Serialize(new { clause_id = "c-1" }),
                                    },
                                },
                            },
                        },
                        finish_reason = "tool_calls",
                    },
                },
                usage = new { prompt_tokens = 20, completion_tokens = 8 },
            };
        }
        else
        {
            body = new
            {
                choices = new[]
                {
                    new
                    {
                        message = new
                        {
                            role = "assistant",
                            content = JsonSerializer.Serialize(new
                            {
                                decision = "non_compliant",
                                rationale = "Retention period of 3 years falls short of the required 7.",
                                confidence = 0.87,
                                citedClauseIds = new[] { "c-1" },
                            }),
                            tool_calls = Array.Empty<object>(),
                        },
                        finish_reason = "stop",
                    },
                },
                usage = new { prompt_tokens = 30, completion_tokens = 6 },
            };
        }

        var json = JsonSerializer.Serialize(body);
        var buffer = Encoding.UTF8.GetBytes(json);
        ctx.Response.ContentType = "application/json";
        ctx.Response.ContentLength64 = buffer.Length;
        await ctx.Response.OutputStream.WriteAsync(buffer);
        ctx.Response.OutputStream.Close();
    }

    private static int _chatCallCount;

    public void Dispose()
    {
        _listener.Stop();
        _listener.Close();
    }
}
