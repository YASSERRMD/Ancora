# Troubleshooting (.NET)

## `DllNotFoundException: libancora_ffi`

The native library is not on the library search path.

**Fix**:

```bash
# Linux
export LD_LIBRARY_PATH=/path/to/native:$LD_LIBRARY_PATH

# macOS
export DYLD_LIBRARY_PATH=/path/to/native:$DYLD_LIBRARY_PATH

# Windows
set PATH=C:\path\to\native;%PATH%
```

## `HttpRequestException: Connection refused`

Ollama is not running.

**Fix**: `ollama serve`

## `TaskCanceledException` on long-running runs

The default `HttpClient` timeout (100 s) is too short.

**Fix**: set a longer timeout in the runtime options:

```csharp
var rt = new Runtime(new RuntimeOptions { HttpTimeout = TimeSpan.FromMinutes(10) });
```

## `JsonException` when parsing structured output

The model returned invalid JSON.

**Fix**: add a retry loop:

```csharp
for (int attempt = 0; attempt < 3; attempt++)
{
    string raw = "";
    await foreach (var ev in agent.Run(spec, prompt).Events())
        if (ev is CompletedEvent c) raw = c.Output;

    try
    {
        return JsonSerializer.Deserialize<MyType>(raw)!;
    }
    catch (JsonException) when (attempt < 2)
    {
        // retry
    }
}
```

## `PolicyViolationException: max_write_tools exceeded`

**Fix**: increase `MaxWriteTools` in `PolicySpec` or reduce write tool calls.

## See also

- [Install](install.md)
- [Durability](durability.md)
