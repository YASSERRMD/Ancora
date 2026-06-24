use std::collections::HashMap;
use std::sync::Mutex;

use crate::error_code::AncorErrorCode;
use crate::handles::AncorRuntime;
use crate::runs::InnerRun;

/// Allocate a new runtime. The caller owns the returned pointer.
/// Returns null on allocation failure.
#[no_mangle]
pub extern "C" fn ancora_create_runtime() -> *mut AncorRuntime {
    let boxed: Box<InnerRuntime> = Box::new(InnerRuntime::new());
    Box::into_raw(boxed).cast()
}

/// Allocate a runtime and write the pointer to `out`. Returns `NullPtr` if `out` is null.
#[no_mangle]
pub extern "C" fn ancora_runtime_new(out: *mut *mut AncorRuntime) -> AncorErrorCode {
    if out.is_null() {
        return AncorErrorCode::NullPtr;
    }
    let boxed: Box<InnerRuntime> = Box::new(InnerRuntime::new());
    unsafe { *out = Box::into_raw(boxed).cast() };
    AncorErrorCode::Ok
}

/// Allocate a runtime with serialized config bytes and write pointer to `out`.
/// Config bytes are currently ignored (reserved for future use).
/// Returns `NullPtr` if `out` is null.
#[no_mangle]
pub extern "C" fn ancora_runtime_new_with_config(
    _config_bytes: *const u8,
    _config_len: usize,
    out: *mut *mut AncorRuntime,
) -> AncorErrorCode {
    ancora_runtime_new(out)
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

pub(crate) struct InnerRuntime {
    pub runs: Mutex<HashMap<String, InnerRun>>,
    _store: ancora_core::journal::MemoryStore,
}

impl InnerRuntime {
    pub fn new() -> Self {
        Self {
            runs: Mutex::new(HashMap::new()),
            _store: ancora_core::journal::MemoryStore::new(),
        }
    }
}
