# Testing Your Agents (.NET)

All Ancora .NET tests run offline by default with xUnit.

## Skip when native library is absent

```csharp
using Ancora;
using Xunit;

public class SingleAgentTests
{
    [Fact]
    public async Task RunsBasicAgent()
    {
        Runtime rt;
        try { rt = new Runtime(); }
        catch (DllNotFoundException) { return; }   // skip if native lib absent

        await using var agent = new Agent(rt);
        var spec = new AgentSpec { Model = "llama3", Instructions = "Answer." };

        string output = "";
        await foreach (var ev in agent.Run(spec, "What is 2+2?").Events())
            if (ev is CompletedEvent c) output = c.Output;

        Assert.NotEmpty(output);
    }
}
```

## Testing tool logic

Tool functions are plain C# delegates. Test them in isolation:

```csharp
[Fact]
public void GetWeatherReturnsCorrectFormat()
{
    var result = GetWeather("Cairo");
    Assert.Contains("Cairo", result);
    Assert.Contains("22 C", result);
}
```

## Testing schema generation

```csharp
[Fact]
public void JsonSchemaGeneratesCorrectly()
{
    var schema = JsonSchemaGenerator.FromType<AnalysisResult>();
    Assert.Equal("object", schema["type"]!.GetString());
    Assert.True(schema["properties"]!.TryGetProperty("headline", out _));
}
```

## Testing with in-memory journal

```csharp
[Fact]
public async Task JournalsARun()
{
    var store = new MemoryStore();
    var rt = new Runtime(new RuntimeOptions { Transport = new StoringTransport(store) });
    await using var agent = new Agent(rt);

    var spec = new AgentSpec { Model = "llama3", Instructions = "Answer." };
    await foreach (var _ in agent.Run(spec, "ping", new RunOptions { RunId = "test-run-1" }).Events()) { }

    Assert.True(store.HasRun("test-run-1"));
}
```

## Running tests

```bash
dotnet test
```

## See also

- [Quickstart](quickstart.md)
- [Durability](durability.md)
