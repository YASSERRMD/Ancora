# Observability and Cost (.NET)

## In-process token tracking

```csharp
using Ancora;

var rt = new Runtime();
await using var agent = new Agent(rt);
var spec = new AgentSpec { Model = "llama3", Instructions = "Answer." };

var totalTokens = 0;
await foreach (var ev in agent.Run(spec, "What is 2+2?").Events())
{
    if (ev is TokenEvent token)
        totalTokens += (int)Math.Ceiling(token.Token.Length / 4.0);
    else if (ev is CompletedEvent completed)
        Console.WriteLine($"Input: {completed.Usage.InputTokens}, Output: {completed.Usage.OutputTokens}");
}

Console.WriteLine($"Estimated output tokens: {totalTokens}");
```

## OpenTelemetry export

```bash
dotnet add package OpenTelemetry.Exporter.OpenTelemetryProtocol
dotnet add package OpenTelemetry.Extensions.Hosting
```

```csharp
using OpenTelemetry.Trace;

builder.Services.AddOpenTelemetry()
    .WithTracing(tracing => tracing
        .AddSource("ancora")
        .AddOtlpExporter(o => o.Endpoint = new Uri("http://localhost:4317")));
```

```csharp
using System.Diagnostics;

var activitySource = new ActivitySource("ancora");

using var activity = activitySource.StartActivity("agent-run");
var spec = new AgentSpec { Model = "llama3", Instructions = "Answer." };

await foreach (var ev in agent.Run(spec, "What is 2+2?").Events())
{
    if (ev is CompletedEvent completed)
    {
        activity?.SetTag("ancora.model", spec.Model);
        activity?.SetTag("ancora.output_tokens", completed.Usage.OutputTokens);
    }
}
```

## See also

- [Durability](durability.md)
- [Policy](policy.md)
