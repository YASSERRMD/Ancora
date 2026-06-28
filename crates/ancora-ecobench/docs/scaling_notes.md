# Scaling Notes

## Registry fetch at scale

The `registry_fetch` module models a linear scan over an in-memory map. In
production the registry may contain thousands of entries.

### Expected behaviour

| Entry count | Expected fetch time (prefix scan) |
|---|---|
| 10 | < 10 us |
| 100 | < 50 us |
| 1 000 | < 200 us |
| 10 000 | < 2 000 us |

The 3 000 us threshold is designed to accommodate up to approximately 5 000
entries before requiring an indexed data structure.

## Catalog install at scale

Installing many packages sequentially is bounded by per-install overhead. The
`catalog_install` module shows that cache hits are effectively free; CI
pipelines should pre-warm the cache to avoid cold-start penalties.

## Adapter overhead scaling

`adapter_overhead` scales with the number of parameters, not the number of
tools. A tool with 100 parameters contributes more overhead than 10 tools with
1 parameter each. Keep tool parameter counts below 20 for best performance.

## Recommendations for large deployments

1. Pre-compile and cache WASM modules at startup.
2. Use persistent subprocess handles for high-frequency plugins.
3. Index the registry by prefix for large catalogs (> 1 000 entries).
4. Batch recipe instantiations where possible to amortise validation cost.
