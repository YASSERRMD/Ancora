# Troubleshooting (Java)

## `UnsatisfiedLinkError: no ancora_ffi in java.library.path`

The native library is not on the JVM library path.

**Fix**:

```bash
java -Djava.library.path=/path/to/native -jar myapp.jar
# or
export LD_LIBRARY_PATH=/path/to/native:$LD_LIBRARY_PATH
```

## `AncoraNative.AVAILABLE` is `false`

The native library could not be loaded at class initialisation.

**Fix**: check that the library ABI matches the JVM platform. The library
name is `libancora_ffi.so` (Linux), `libancora_ffi.dylib` (macOS), or
`ancora_ffi.dll` (Windows).

## `ConnectException: Connection refused` when calling Ollama

Ollama is not running.

**Fix**: `ollama serve`

## `PolicyViolationException`

The agent exceeded the `maxWriteTools` limit or called a denied provider.

**Fix**: increase `maxWriteTools` in `PolicySpec` or reduce write tool calls.

## `JsonProcessingException` when parsing structured output

The model returned invalid JSON.

**Fix**: add a retry loop:

```java
ObjectMapper mapper = new ObjectMapper();
for (int attempt = 0; attempt < 3; attempt++) {
    String raw = "";
    for (var ev : agent.run(spec, prompt).events())
        if (ev instanceof RunEvent.Completed c) raw = c.output();
    try {
        return mapper.readValue(raw, MyType.class);
    } catch (JsonProcessingException e) {
        if (attempt == 2) throw e;
    }
}
```

## `OutOfMemoryError` during embedding

The embedding model requires too much heap.

**Fix**: increase heap size: `java -Xmx4g -jar myapp.jar`

## See also

- [Install](install.md)
- [Durability](durability.md)
