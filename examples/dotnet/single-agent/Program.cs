using Ancora;

Console.WriteLine("Ancora .NET SDK: single-agent example");
Console.WriteLine("======================================");
Console.WriteLine();

// Build an agent spec targeting a local Ollama-compatible endpoint.
// Switch the model to any OpenAI-compatible name served by your local endpoint.
var spec = new AgentSpec(
    Model: "llama3",
    Instructions: "You are a concise assistant. Answer in one sentence.",
    MaxTokens: 256,
    Temperature: 0.7
);

// Create an Agent backed by a new runtime.
using var agent = new Agent();

Console.WriteLine($"Spec: model={spec.Model} max_tokens={spec.MaxTokens}");
Console.WriteLine();

// Start the run. The agent connects to the local inference endpoint.
var handle = agent.Run(spec);
Console.WriteLine($"Run ID: {handle.RunId}");
Console.WriteLine();

// Stream events as they arrive.
Console.Write("Response: ");
await foreach (var ev in handle.EventsAsync())
{
    switch (ev)
    {
        case StartedEvent:
            break;
        case TokenEvent t:
            Console.Write(t.Text);
            break;
        case CompletedEvent completed:
            Console.Write(completed.Output);
            Console.WriteLine();
            Console.WriteLine();
            Console.WriteLine("Run completed.");
            break;
        case FailedEvent failed:
            Console.WriteLine();
            Console.WriteLine($"Run failed: {failed.Error}");
            break;
        case ToolCallEvent tc:
            Console.WriteLine($"[tool call: {tc.Name}]");
            break;
    }
}

// Print cost summary.
var cost = handle.GetCostTyped();
Console.WriteLine($"Cost: ${cost.TotalUsd:F6}");
