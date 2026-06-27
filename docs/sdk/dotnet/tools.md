# Defining Tools (.NET)

Tools are delegates registered with a `ToolRegistry` and passed to an
`AgentSpec`. The agent calls them during a run; return values are injected
back into the model context.

## Register a tool

```csharp
using Ancora;

var registry = new ToolRegistry();

registry.Register(new ToolSpec
{
    Name = "get_weather",
    Description = "Return the current weather for a city.",
    InputSchema = new ToolInputSchema
    {
        Type = "object",
        Properties = new Dictionary<string, ToolInputProperty>
        {
            ["city"] = new ToolInputProperty { Type = "string", Description = "City name" }
        },
        Required = new List<string> { "city" }
    },
    Fn = args =>
    {
        var city = args["city"]!.GetString()!;
        return $"{city}: 22 C, sunny";
    }
});

var spec = new AgentSpec
{
    Model = "llama3",
    Instructions = "Use get_weather to answer weather questions.",
    Tools = registry.ToList(),
};
```

## Async tools

```csharp
registry.Register(new ToolSpec
{
    Name = "fetch_url",
    Description = "Fetch the text content of a URL.",
    InputSchema = new ToolInputSchema
    {
        Type = "object",
        Properties = new Dictionary<string, ToolInputProperty>
        {
            ["url"] = new ToolInputProperty { Type = "string" }
        },
        Required = new List<string> { "url" }
    },
    AsyncFn = async args =>
    {
        var url = args["url"]!.GetString()!;
        using var client = new HttpClient();
        var text = await client.GetStringAsync(url);
        return text[..Math.Min(500, text.Length)];
    }
});
```

## Effect classes

```csharp
registry.Register(new ToolSpec
{
    Name = "write_file",
    Description = "Write content to a file.",
    Effect = EffectClass.Write,
    // ...
    Fn = args => { /* write file */ return "ok"; }
});
```

## See also

- [Structured output](structured-output.md)
- [Policy](policy.md)
