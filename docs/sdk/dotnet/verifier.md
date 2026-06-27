# Verifier and Consensus (.NET)

## Simple verifier

```csharp
using Ancora;

var rt = new Runtime();
await using var agent = new Agent(rt);

var graph = new GraphSpec
{
    Nodes = new List<GraphNode>
    {
        new() { Id = "primary", Spec = new AgentSpec { Model = "llama3", Instructions = "Answer the question." } },
        new()
        {
            Id = "verifier",
            Spec = new AgentSpec
            {
                Model = "llama3",
                Instructions = "Verify the previous answer. Reply 'VERIFIED' or 'REJECTED: <reason>'.",
            }
        },
    },
    Edges = new List<GraphEdge> { new() { From = "primary", To = "verifier" } },
};

await foreach (var ev in agent.RunGraph(graph, "What is the capital of Egypt?").Events())
{
    if (ev is CompletedEvent completed)
        Console.WriteLine(completed.Output);
}
```

## N-verifier consensus

```csharp
var primarySpec = new AgentSpec { Model = "llama3", Instructions = "Answer the question." };
var verifierSpec = new AgentSpec { Model = "llama3", Instructions = "Is this answer correct? Reply YES or NO." };

var handle = agent.Run(primarySpec, "What is the capital of Egypt?");
string candidate = "";
await foreach (var ev in handle.Events())
    if (ev is CompletedEvent c) candidate = c.Output;

var tasks = Enumerable.Range(0, 3).Select(async _ =>
{
    string verdict = "";
    await foreach (var ev in agent.Run(verifierSpec, candidate).Events())
        if (ev is CompletedEvent c) verdict = c.Output;
    return verdict.Trim().StartsWith("YES", StringComparison.OrdinalIgnoreCase);
});

var verdicts = await Task.WhenAll(tasks);
Console.WriteLine(verdicts.Count(v => v) >= 2 ? "ACCEPTED" : "REJECTED");
```

## See also

- [Multi-agent graphs](multi-agent.md)
- [Human-in-the-loop](human-in-the-loop.md)
