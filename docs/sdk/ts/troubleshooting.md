# Troubleshooting (TypeScript)

## `Error: Cannot find module 'ancora'`

The package is not installed.

**Fix**: `npm install ancora`

## `Error: libancora_ffi.so: cannot open shared object file`

The native N-API addon cannot find the native library at runtime.

**Fix**:

```bash
export LD_LIBRARY_PATH=/path/to/target/release:$LD_LIBRARY_PATH
```

Or install the pre-built npm package which bundles the library.

## `fetch failed` / `ECONNREFUSED` when calling Ollama

Ollama is not running.

**Fix**:

```bash
ollama serve
```

## `TypeError: rt.stream is not a function`

You are using an older version of the SDK that does not support streaming.

**Fix**: `npm install ancora@latest`

## `ZodError` on `result.parse(schema)`

The model returned output that does not match the Zod schema.

**Fix**: add a retry loop:

```ts
for (let attempt = 0; attempt < 3; attempt++) {
  const result = await rt.run(spec, prompt)
  try {
    return result.parse(MySchema)
  } catch (e) {
    if (attempt === 2) throw e
  }
}
```

## `PolicyViolationError: max_write_tools exceeded`

The agent called more write-effect tools than the policy allows.

**Fix**: increase `maxWriteTools` in the `policy` option or reduce write tool calls.

## `Cannot use import statement` in CommonJS context

You are importing the ESM build in a CJS context.

**Fix**: use the CJS import or add `"type": "module"` to `package.json`.

## See also

- [Install](install.md)
- [Durability](durability.md)
