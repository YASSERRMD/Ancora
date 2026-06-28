# Known Limitations

## LIM-001: Plugin hot-reload pause

Plugin hot-reload requires a brief pause between consecutive reloads.

- Workaround: Wait 500ms between consecutive reloads
- Target release: v0.7.0

## LIM-002: gRPC streaming backpressure

gRPC streaming backpressure is not yet propagated to callers.

- Workaround: Use chunked polling as an interim approach
- Target release: v0.7.0

## LIM-003: Python FFI async generators

The Python FFI layer does not support async generators.

- Workaround: Use synchronous iteration with manual polling
- Target release: v0.8.0

## LIM-004: Catalog search pagination

Catalog search results are limited to 100 entries per page.

- Workaround: None currently
- Target release: v0.7.0
