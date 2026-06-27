# Troubleshooting (Go)

## `cannot find -lancora_ffi`

The CGo linker cannot find the native library.

**Fix**: ensure `CGO_LDFLAGS` points to the directory containing
`libancora_ffi.so` / `libancora_ffi.dylib`, and that
`CGO_CFLAGS` points to the directory containing `ancora_ffi.h`.

```bash
export CGO_LDFLAGS="-L/path/to/target/release -lancora_ffi"
export LD_LIBRARY_PATH=/path/to/target/release:$LD_LIBRARY_PATH
```

## `runtime: out of memory` during embedding

The embedding model requires more RAM than is available.

**Fix**: use `HashEmbedder` for offline tests, or reduce the chunk size with
`PipelineConfig.ChunkSize`.

## `connection refused` when calling Ollama

Ollama is not running.

**Fix**: start Ollama with `ollama serve`, then verify with
`curl http://localhost:11434/api/version`.

## `model not found: llama3`

The model weight has not been pulled.

**Fix**: `ollama pull llama3`.

## Events are empty after `CollectAll`

The run failed silently.

**Fix**: check the last event -- if it is an `ErrorEvent`, print its message:

```go
last := events[len(events)-1]
if errEv, ok := last.(*ancora.ErrorEvent); ok {
    fmt.Println("run failed:", errEv.Message)
}
```

## `DuplicateActivityKey` journal error on replay

An activity key collision was detected. Usually caused by a broken
idempotency key template.

**Fix**: ensure `ToolSpec.IdempotencyKeyTemplate` includes `{seq}` to
differentiate repeated calls to the same tool within one run.

## See also

- [Install](install.md)
- [Durability](durability.md)
