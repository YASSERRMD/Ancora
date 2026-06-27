# Testing Your Agents (Go)

All Ancora Go tests run offline by default. No live API keys or network
connections are needed.

## Offline test pattern

Use `t.Skip` when the native library is absent:

```go
func TestSingleAgent(t *testing.T) {
    agent, err := ancora.NewAgent()
    if err != nil {
        t.Skip("native library not available:", err)
    }
    defer agent.Close()

    spec := ancora.NewAgentSpec("local-model", "Respond.")
    events, err := agent.Run(spec).CollectAll()
    require.NoError(t, err)
    require.NotEmpty(t, events)
}
```

## Testing tool logic

Tool functions are plain Go functions; test them without an agent:

```go
func TestGetWeather(t *testing.T) {
    result := getWeather("Cairo")
    assert.Contains(t, result, "Cairo")
    assert.Contains(t, result, "22 C")
}
```

## Testing schema generation

```go
func TestSchemaFromStruct(t *testing.T) {
    schema := ancora.SchemaFromStruct(AnalysisResult{})
    assert.Equal(t, "object", schema["type"])
    assert.Contains(t, schema["required"], "summary")
}
```

## Running the example tests

```bash
cd sdk/go/examples
go test ./... -v
```

## See also

- [Quickstart](quickstart.md)
- [Go examples](../../../sdk/go/examples/)
