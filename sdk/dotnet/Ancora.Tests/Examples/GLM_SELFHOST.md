# GLM Self-Host Example

Demonstrates iterating over multiple GLM model variants using the same
`Agent` instance and verifying that each run receives a distinct `RunId`.

## What it tests

- All four model names are distinct strings
- An `AgentSpec` can be built for each GLM model variant
- Each `RunHandle.RunId` is unique across all variants

## Pattern

```csharp
var glmModels = new[] { "glm-4", "glm-4-flash", "glm-4-air", "glm-3-turbo" };

using var agent = new Agent();
var runIds = new List<string>();

foreach (var model in glmModels)
{
    var spec = new AgentSpec(model, "Respond briefly.");
    var handle = agent.Run(spec);
    runIds.Add(handle.RunId);
    await handle.CollectAsync();
}

// All run IDs are distinct
Assert.Equal(runIds.Count, runIds.Distinct().Count());
```

## Offline behaviour

The model-name and run-ID tests are in-process. The agent run loop catches
`DllNotFoundException` and exits cleanly.
