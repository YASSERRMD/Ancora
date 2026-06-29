# Footprint Tuning

Reducing binary size and runtime memory on constrained ARM and mobile devices
requires a layered approach: compile-time settings, feature selection, and
model quantisation.

## Binary Size

### 1. Use the `ondev` build profile

```toml
[profile.ondev]
inherits      = "release"
opt-level     = "z"      # optimise for size, not speed
panic         = "abort"  # removes unwinding tables
lto           = true     # full LTO removes dead code across crates
strip         = true     # remove debug symbols from the output binary
codegen-units = 1        # enables maximum inter-procedural optimisation
```

### 2. Disable unused features

```bash
cargo build --profile ondev \
    --no-default-features \
    --features minimal,sqlite_journal
```

The `FeatureRegistry` reports the estimated size contribution of each feature:

```rust
let r = FeatureRegistry::minimal();
println!("enabled features: {} bytes", r.total_size_bytes());
```

Call `trim_to_budget(budget_bytes)` to automatically disable low-priority
features until the estimate fits within your target.

### 3. Prefer `musl` over `glibc` on Linux

`aarch64-unknown-linux-musl` produces fully-static binaries that do not pull
in glibc symbols, reducing the effective download size on Android sideloads
and embedded Linux targets.

### 4. Compress the binary

After stripping, run `upx` (Ultimate Packer for eXecutables):

```bash
upx --ultra-brute target/aarch64-unknown-linux-musl/ondev/ancora-ondev-cli
```

Expect 40-60% compression on typical Rust binaries.

---

## Runtime Memory

### 1. Measure first

```rust
let snap = ancora_ondev::perf::MemorySnapshot::capture();
println!("RSS: {} KiB", snap.rss_kib());
```

On Linux and Android `/proc/self/statm` is read; on iOS use `task_info`.

### 2. Limit the vector store

The embedded `MemoryStore` keeps all records in a `HashMap`.  For long-running
agents, periodically evict old records:

```rust
// Keep only records for the current session.
let to_remove: Vec<String> = store
    .records_for_agent("agent-1")
    .iter()
    .filter(|r| r.metadata["session"] != current_session)
    .map(|r| r.id.clone())
    .collect();
for id in to_remove { store.delete(&id); }
```

### 3. Use a quantised model

4-bit GGUF models (Q4_K_M) use roughly 4x less RAM than FP16 weights.
A 7B-parameter model at Q4_K_M requires about 4 GiB of address space but
only ~3.5 GiB of RSS; a 1B model at Q4_K_M fits in 800 MiB.

### 4. Set stack size explicitly

On Android and iOS the default stack is 1-8 MiB.  If your agent uses deep
recursion, spawn worker threads with an explicit stack:

```rust
std::thread::Builder::new()
    .stack_size(4 * 1024 * 1024)
    .spawn(|| { /* agent work */ })
    .unwrap();
```

---

## Target Size Budgets

| Platform | Recommended binary budget | Recommended RSS budget |
|----------|--------------------------|------------------------|
| Android APK | 8 MiB (compressed) | 256 MiB |
| iOS IPA | 10 MiB | 256 MiB |
| Embedded Linux (Pi) | 5 MiB | 128 MiB |
| ARM32 MCU | 1 MiB | 32 MiB |

Adjust the `size_budget_bytes` field of `BuildProfile` to match your target
and call `within_budget(actual_bytes)` in CI to catch regressions.
