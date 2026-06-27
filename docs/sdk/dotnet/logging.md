# Logging and Diagnostics (.NET)

## Built-in tracing

Ancora uses the Rust `tracing` crate internally. Set `ANCORA_LOG_LEVEL` to
control verbosity:

```bash
ANCORA_LOG_LEVEL=debug dotnet run
```

| Level | Description |
|-------|-------------|
| `error` | Fatal errors only |
| `warn` | Unexpected conditions (default) |
| `info` | Activity boundaries and run lifecycle |
| `debug` | Every activity recorded and replayed |
| `trace` | FFI boundary crossings |

## Microsoft.Extensions.Logging integration

```csharp
using Microsoft.Extensions.Logging;

var loggerFactory = LoggerFactory.Create(b => b.AddConsole().SetMinimumLevel(LogLevel.Debug));
var rt = new Runtime(new RuntimeOptions { LoggerFactory = loggerFactory });
```

Ancora forwards internal log messages to the .NET `ILogger` infrastructure
when a `LoggerFactory` is provided.

## Activity tracing with `System.Diagnostics`

```csharp
using System.Diagnostics;

var source = new ActivitySource("ancora");

using var activity = source.StartActivity("agent-run");
activity?.SetTag("ancora.model", spec.Model);

await foreach (var ev in agent.Run(spec, prompt).Events())
{
    if (ev is CompletedEvent c)
    {
        activity?.SetTag("ancora.output_tokens", c.Usage.OutputTokens);
        Console.WriteLine(c.Output);
    }
}
```

## Structured logs

Enable JSON structured logging via `Serilog` or `NLog`:

```csharp
Log.Logger = new LoggerConfiguration()
    .WriteTo.Console(outputTemplate: "{Timestamp} [{Level}] {Message:lj}{NewLine}{Exception}")
    .CreateLogger();

var loggerFactory = new SerilogLoggerFactory();
var rt = new Runtime(new RuntimeOptions { LoggerFactory = loggerFactory });
```

## See also

- [Observability](observability.md)
- [Configuration](configuration.md)
