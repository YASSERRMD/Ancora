# MCP Tool Use Example

Demonstrates defining `ToolSpec` objects with `ToolInputSchema`, wiring them
into an `AgentSpec`, and verifying local function dispatch logic.

## What it tests

- Local functions (`GetWeather`, `Calculate`) return correct values
- `ToolSpec` carries the expected `Name`, `Description`, and `InputSchema`
- `InputSchema.Properties` and `Required` are populated correctly
- `AgentSpec` constructed with `List<ToolSpec>` runs without error

## Pattern

```csharp
static string GetWeather(string location)
    => $"Weather in {location}: 22 C, partly cloudy";

var weatherSpec = new ToolSpec(
    Name: "get_weather",
    Description: "Get weather for a location.",
    InputSchema: new ToolInputSchema(
        Type: "object",
        Properties: new Dictionary<string, ToolInputProperty>
        {
            ["location"] = new("string", "City name"),
        },
        Required: new List<string> { "location" }
    )
);

// AgentSpec accepts List<ToolSpec>, not ToolSpec[]
var agentSpec = new AgentSpec("local-model", "Use tools.",
    new List<ToolSpec> { weatherSpec });
```

## Offline behaviour

Tool function tests run entirely in-process. The agent run catches
`DllNotFoundException`.
