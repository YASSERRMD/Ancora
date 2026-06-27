# Deployment (.NET)

## Self-contained single binary

```bash
dotnet publish -c Release -r linux-x64 --self-contained true \
    -p:PublishSingleFile=true -o publish/
cp target/release/libancora_ffi.so publish/
```

Set `LD_LIBRARY_PATH` at runtime:

```bash
LD_LIBRARY_PATH=./publish ./publish/MyAgent
```

## Docker

```dockerfile
FROM mcr.microsoft.com/dotnet/runtime:8.0-slim

RUN apt-get update && apt-get install -y libgcc-s1 && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY publish/ ./

ENV ANCORA_MODEL_URL=http://ollama:11434
ENV LD_LIBRARY_PATH=/app

ENTRYPOINT ["./MyAgent"]
```

## ASP.NET Core API

```csharp
// Program.cs
var builder = WebApplication.CreateBuilder(args);
builder.Services.AddSingleton<Runtime>();
builder.Services.AddScoped<Agent>(sp => new Agent(sp.GetRequiredService<Runtime>()));

var app = builder.Build();

app.MapPost("/ask", async (Agent agent, string prompt) =>
{
    var spec = new AgentSpec { Model = "llama3", Instructions = "Answer." };
    string output = "";
    await foreach (var ev in agent.Run(spec, prompt).Events())
        if (ev is CompletedEvent c) output = c.Output;
    return Results.Ok(new { output });
});

app.Run();
```

## Air-gapped deployment

1. Build a NuGet offline package:
   ```bash
   dotnet pack -o ./nupkgs
   ```
2. Copy `./nupkgs/` and the native library to the air-gapped host.
3. Configure a local NuGet source:
   ```bash
   dotnet nuget add source ./nupkgs --name local
   dotnet restore --source local
   ```
4. Install Ollama and pull the model weight on the air-gapped host.

## See also

- [Install](install.md)
- [Deployment models concept](../../concepts/deployment-models.md)
