# model hot-swap guide

ancora-swap lets you replace the model backing an agent runtime without
stopping or restarting any in-flight runs.

## quick start

```rust
use ancora_swap::model::{ModelHandle, ModelMeta, ModelVersion};
use ancora_swap::runtime::{RunId, SwapRuntime};

// 1. create the initial runtime
let m1 = ModelHandle::new(meta_v1, ModelVersion::next());
let rt  = SwapRuntime::new(m1);

// 2. start a run (pins the current model)
rt.start_run(RunId(1));

// 3. warm up a candidate
let m2 = ModelHandle::new(meta_v2, ModelVersion::next());
rt.warmup(&m2, 0); // 0 ms in tests

// 4. swap -- existing run keeps its pin on m1, new runs get m2
rt.swap(m2);

// 5. finish old run; memory is reclaimed automatically
rt.finish_run(RunId(1));
rt.reclaim_unloaded();
```

## how it works

1. `start_run` atomically increments the pin count on the active model and
   stores a `ModelPin` guard inside the runtime.
2. `swap` marks the old model as unloaded (blocking new pins) and moves it to
   the drain slot.  The new model becomes active.
3. In-flight runs continue to reference the old model via their stored pin.
4. Once the last in-flight run calls `finish_run` the pin count falls to zero
   and `reclaim_unloaded` (or `finish_run` itself) releases the drain slot.

## failure modes

* warmup failure -- warmup returns `WarmupStatus::Failed`; do not call `swap`.
* rollback -- if a swap causes problems, call `rt.rollback()` within the same
  process lifetime to restore the previous model.
* memory pressure -- keep the drain window short by draining old runs quickly.
