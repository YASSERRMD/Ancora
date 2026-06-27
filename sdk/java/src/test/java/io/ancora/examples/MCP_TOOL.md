# MCP Tool Use Example

Demonstrates defining `ToolSpec` objects with `ToolInputSchema`, wiring them
into an `AgentSpec`, and verifying local function dispatch logic.

## What it tests

- Local static methods (`getWeather`, `calculate`) return correct values
- `ToolSpec` carries the expected `name`, `description`, and `inputSchema`
- `ToolInputSchema.properties()` and `required()` are populated correctly
- `AgentSpec` constructed with `List<ToolSpec>` runs without error

## Pattern

```java
static String getWeather(String location) {
    return "Weather in " + location + ": 22 C, partly cloudy";
}

ToolSpec weatherSpec = new ToolSpec(
    "get_weather",
    "Get weather for a location.",
    new ToolInputSchema(
        "object",
        Map.of("location", new ToolInputProperty("string", "City name")),
        List.of("location")
    )
);

AgentSpec spec = new AgentSpec("local-model", "Use tools.", List.of(weatherSpec), null, null);
```

## Offline behaviour

Tool function tests run entirely in-process. Agent run catches
`UnsatisfiedLinkError`.
