# version pinning

version pinning ensures a run always uses the model it started with, even
after one or more hot-swaps occur during its lifetime.

## automatic pinning (built into the runtime)

when you call `SwapRuntime::start_run(run_id)` the runtime:

1. reads the currently active `ModelHandle`
2. calls `handle.pin()` to atomically increment the reference count
3. stores the resulting `ModelPin` guard under `run_id`

the pin is released when `finish_run(run_id)` is called.

## explicit pinning (PinRegistry)

for workflows that need to pre-assign a model before the run starts use
`PinRegistry`:

```rust
use ancora_swap::pin::PinRegistry;
use ancora_swap::runtime::RunId;

let reg = PinRegistry::new();
reg.pin_run(RunId(42), preferred_handle.clone());

// later, look up which model the run should use
let h = reg.get(RunId(42)).unwrap();
```

## checking the pinned version

```rust
// after start_run
let version = rt.run_model_version(RunId(42));
assert_eq!(version, Some(original_handle.version()));
```

## why pinning matters

without pinning, a run could switch to the new model mid-inference if a swap
occurs, producing inconsistent outputs.  pinning gives each run a stable,
immutable view of the model for its entire lifetime.
