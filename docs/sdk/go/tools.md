# Defining Tools

Register tools with `GoToolRegistry` and attach them to an `AgentSpec`.

## Registering a tool

```go
registry := ancora.NewGoToolRegistry()

registry.Register("get_weather", ancora.ToolSpec{
    Description: "Get the current weather for a city.",
    InputSchema: ancora.SchemaFromStruct(struct {
        City string `json:"city" description:"City name"`
    }{}),
}, func(input map[string]any) (any, error) {
    city := input["city"].(string)
    return fmt.Sprintf("Weather in %s: 22 C, partly cloudy", city), nil
})

spec := ancora.NewAgentSpec("llama3", "Use tools to answer questions.")
spec.Tools = registry.Specs()

agent, _ := ancora.NewAgent()
defer agent.Close()
run, _ := agent.RunWithTools(spec, registry)
```

## Unregistering a tool

```go
registry.Unregister("get_weather")
fmt.Println("has tool:", registry.Has("get_weather")) // false
fmt.Println("count:", registry.Count())               // 0
```

## RuntimeToolkit

`RuntimeToolkit` wraps a registry and a runtime for convenience:

```go
toolkit := ancora.NewRuntimeToolkit(runtime, registry)
handle, _ := toolkit.Run(spec)
```

## Input schema

`SchemaFromStruct` generates a JSON Schema from a Go struct using `json`
tags. Fields tagged with `description` get a description in the schema.

## See also

- [Quickstart](quickstart.md)
- [MCP and A2A](mcp-and-a2a.md)
