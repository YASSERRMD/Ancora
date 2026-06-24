use crate::handles::AncorRuntime;

/// Allocate a new runtime. The caller owns the returned pointer.
/// Returns null on allocation failure.
#[no_mangle]
pub extern "C" fn ancora_create_runtime() -> *mut AncorRuntime {
    let boxed: Box<InnerRuntime> = Box::new(InnerRuntime::new());
    Box::into_raw(boxed).cast()
}

/// Free a runtime previously created by `ancora_create_runtime`.
/// Passing null is a no-op.
#[no_mangle]
pub extern "C" fn ancora_free_runtime(ptr: *mut AncorRuntime) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        drop(Box::from_raw(ptr.cast::<InnerRuntime>()));
    }
}

struct InnerRuntime {
    _store: ancora_core::journal::MemoryStore,
}

impl InnerRuntime {
    fn new() -> Self {
        Self { _store: ancora_core::journal::MemoryStore::new() }
    }
}
