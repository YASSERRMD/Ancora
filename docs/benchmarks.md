# Ancora Benchmark Methodology and Results

## Methodology

All benchmarks use [Criterion.rs](https://github.com/bheisler/criterion.rs) version 0.5.
Criterion measures wall-clock time per iteration with statistical outlier
removal and warm-up phases.

### Environment

Benchmarks are run on a quiet machine (no background workloads) with:

- **CPU**: as reported by `/proc/cpuinfo` or `sysctl -n machdep.cpu.brand_string`
- **OS**: see `uname -a`
- **Rust**: as reported by `rustc --version`
- **Profile**: `--release` (optimised, without debug symbols)

### Running benchmarks

```bash
# Engine overhead and replay
cargo bench -p ancora-core

# FFI call overhead
cargo bench -p ancora-ffi

# All benchmarks with HTML report
cargo bench --workspace
```

HTML reports are written to `target/criterion/`.

## Benchmark suites

### encore-core: graph validation

| Nodes | Time (approx) |
|-------|---------------|
| 1     | < 1 us        |
| 4     | < 1 us        |
| 16    | < 1 us        |
| 64    | < 5 us        |

`Graph::validate()` performs edge-reachability and cycle detection.
Complexity is O(V + E) where V is node count and E is edge count.

### ancora-core: replay

| Nodes | Events | Time (approx) |
|-------|--------|---------------|
| 1     | 3      | < 1 us        |
| 4     | 11     | < 1 us        |
| 16    | 35     | < 5 us        |
| 64    | 131    | < 20 us       |

`replay_events` folds all journal events into a `ReplayState`.
Complexity is O(E) in event count.

### ancora-core: journal append throughput

| Events | Time per batch (approx) |
|--------|------------------------|
| 10     | < 5 us                 |
| 100    | < 50 us                |
| 1 000  | < 500 us               |
| 10 000 | < 5 ms                 |

`MemoryStore::append` holds a mutex per call.  For the in-memory store,
throughput is dominated by mutex contention and allocation.
A SQLite-backed store has higher per-call latency but durable writes.

### ancora-ffi: FFI call overhead

| Operation              | Time (approx) |
|------------------------|---------------|
| `ancora_version()`     | < 100 ns      |
| `ancora_runtime_new()` | < 50 us       |

`ancora_version()` returns a static C string with no allocation.
`ancora_runtime_new()` creates a new `Runtime` including journal and
executor state; the one-time allocation cost is amortised over the run
lifetime.

## Interpretation

These numbers represent the framework overhead **excluding** model inference
latency.  In a real workload, model inference dominates (typically 100ms-10s
per turn), making the framework overhead negligible.

The journal append throughput is the primary scaling constraint for high-
event-rate scenarios (many tool calls or many nodes in a single run).
The SQLite journal handles up to ~10 000 appends/second on a single-core
spin; PostgreSQL is appropriate for multi-process or high-throughput
deployments.

## Updating these results

After any significant change to the engine, replay, or journal:

```bash
cargo bench -p ancora-core > docs/bench-results-$(date +%Y%m%d).txt
```

Compare against the previous baseline with:

```bash
cargo bench -p ancora-core -- --baseline previous
```

Save the HTML report to `docs/criterion/` for historical reference.
