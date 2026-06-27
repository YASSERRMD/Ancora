# Source Generation (.NET)

Ancora supports Roslyn source generators to produce boilerplate-free tool
registrations and JSON schema definitions at compile time.

## Tool registration source generator

Add `Ancora.Generators` to your project:

```bash
dotnet add package Ancora.Generators
```

Decorate methods with `[AncoraTool]`:

```csharp
using Ancora.Generators;

public partial class WeatherTools
{
    [AncoraTool(Description = "Return the current weather for a city.")]
    public static string GetWeather(string city) => $"{city}: 22 C, sunny";

    [AncoraTool(Description = "Look up a product price.", Effect = EffectClass.Read)]
    public static string GetPrice(string sku, string currency = "USD") => $"{sku}: 29.99 {currency}";
}
```

The generator produces a `WeatherTools.Register(ToolRegistry registry)` extension
method at compile time. Register all tools with a single call:

```csharp
var registry = new ToolRegistry();
WeatherTools.Register(registry);

var spec = new AgentSpec
{
    Model = "llama3",
    Instructions = "Answer questions about weather and prices.",
    Tools = registry.ToList(),
};
```

## JSON schema source generator

Use the built-in `System.Text.Json` source generator for native AOT
compatibility:

```csharp
using System.Text.Json.Serialization;

[JsonSerializable(typeof(AnalysisResult))]
[JsonSerializable(typeof(Report))]
public partial class AncorSerializerContext : JsonSerializerContext { }
```

Pass the context to `JsonSchemaGenerator`:

```csharp
var schema = JsonSchemaGenerator.FromType<AnalysisResult>(AncorSerializerContext.Default);
```

## Native AOT support

Ancora's .NET SDK is compatible with NativeAOT when source generators are
used for both tool registration and JSON serialisation. Add to your `.csproj`:

```xml
<PropertyGroup>
  <PublishAot>true</PublishAot>
</PropertyGroup>
```

## See also

- [Tools](tools.md)
- [Structured output](structured-output.md)
- [Deployment](deployment.md)
