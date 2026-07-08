use crate::model::ModelHandle;
/// Tests verifying memory is reclaimed after unload.
use crate::model::{ModelMeta, ModelVersion};
use crate::reclaim::ReclaimQueue;
use crate::runtime::{make_model, SwapRuntime};

fn make_handle_with_bytes(name: &str, bytes: u64) -> ModelHandle {
    let v = ModelVersion::next();
    ModelHandle::new(
        ModelMeta {
            name: name.to_string(),
            version: "0.1.0".to_string(),
            memory_bytes: bytes,
        },
        v,
    )
}

#[test]
fn test_reclaim_queue_sweep() {
    let mut q = ReclaimQueue::new();
    let h = make_handle_with_bytes("big", 1024 * 1024);
    h.unload();
    q.enqueue(h);

    assert_eq!(q.pending_count(), 1);
    let freed = q.sweep();
    assert_eq!(freed, 1);
    assert_eq!(q.pending_count(), 0);
}

#[test]
fn test_reclaim_queue_waits_for_pin() {
    let mut q = ReclaimQueue::new();
    let h = make_handle_with_bytes("pinned", 512);
    let _pin = h.pin().unwrap();
    h.unload();
    q.enqueue(h);

    let freed = q.sweep();
    assert_eq!(freed, 0, "pin still held, cannot reclaim");
    assert_eq!(q.pending_count(), 1);

    // Drop pin, then sweep.
    drop(_pin);
    let freed = q.sweep();
    assert_eq!(freed, 1);
}

#[test]
fn test_reclaim_queue_pending_bytes() {
    let mut q = ReclaimQueue::new();
    let h1 = make_handle_with_bytes("a", 100);
    let h2 = make_handle_with_bytes("b", 200);
    h1.unload();
    h2.unload();
    q.enqueue(h1);
    q.enqueue(h2);
    assert_eq!(q.pending_bytes(), 300);
}

#[test]
fn test_runtime_memory_reclaimed_after_unload() {
    let m1 = make_model("reclaim-base");
    let rt = SwapRuntime::new(m1);

    let m2 = make_model("reclaim-new");
    rt.swap(m2);

    // No in-flight runs on old model.
    let freed = rt.reclaim_unloaded();
    assert_eq!(freed, 1);
    assert!(!rt.is_draining());
}
