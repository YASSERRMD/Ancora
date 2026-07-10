use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::error_code::AncorErrorCode;
use crate::handles::AncorRuntime;
use crate::model_client::ModelBackend;
use crate::runs::InnerRun;
use crate::tool_registry::ToolRegistry;

/// Allocate a new runtime. The caller owns the returned pointer.
/// Returns null on allocation failure.
///
/// # Safety
/// The caller must eventually free the returned pointer exactly once with
/// `ancora_free_runtime` (if non-null), and must not use it afterward.
#[no_mangle]
pub unsafe extern "C" fn ancora_create_runtime() -> *mut AncorRuntime {
    let boxed: Box<InnerRuntime> = Box::default();
    Box::into_raw(boxed).cast()
}

/// Allocate a runtime and write the pointer to `out`. Returns `NullPtr` if `out` is null.
///
/// # Safety
/// `out` must point to valid, writable memory for a pointer.
#[no_mangle]
pub unsafe extern "C" fn ancora_runtime_new(out: *mut *mut AncorRuntime) -> AncorErrorCode {
    if out.is_null() {
        return AncorErrorCode::NullPtr;
    }
    let boxed: Box<InnerRuntime> = Box::default();
    unsafe { *out = Box::into_raw(boxed).cast() };
    AncorErrorCode::Ok
}

/// Allocate a runtime with serialized config bytes and write pointer to `out`.
///
/// Config bytes are JSON: `{"provider":{"base_url":"...","auth_env_var":"...",
/// "chat_completions_path":"..."}}`. `base_url` points at any
/// OpenAI-compatible chat-completions endpoint (hosted or self-hosted, e.g.
/// NVIDIA NIM); switching is a `base_url` change only. Missing, empty, or
/// unrecognized config bytes fall back to the offline echo model client used
/// by `ancora_runtime_new`, so this never fails on malformed input.
/// Returns `NullPtr` if `out` is null.
///
/// # Safety
/// `out` must point to valid, writable memory for a pointer. If `config_bytes`
/// is non-null it must point to at least `config_len` valid bytes.
#[no_mangle]
pub unsafe extern "C" fn ancora_runtime_new_with_config(
    config_bytes: *const u8,
    config_len: usize,
    out: *mut *mut AncorRuntime,
) -> AncorErrorCode {
    if out.is_null() {
        return AncorErrorCode::NullPtr;
    }
    let bytes = if config_bytes.is_null() || config_len == 0 {
        &[][..]
    } else {
        unsafe { std::slice::from_raw_parts(config_bytes, config_len) }
    };
    let boxed: Box<InnerRuntime> = Box::new(InnerRuntime::with_model_backend(
        ModelBackend::from_config_bytes(bytes),
    ));
    unsafe { *out = Box::into_raw(boxed).cast() };
    AncorErrorCode::Ok
}

/// Free a runtime previously created by `ancora_create_runtime`.
/// Passing null is a no-op.
///
/// # Safety
/// `ptr` must have been returned by `ancora_create_runtime`/`ancora_runtime_new`
/// (or be null), must not be freed more than once, and must not be used afterward.
#[no_mangle]
pub unsafe extern "C" fn ancora_free_runtime(ptr: *mut AncorRuntime) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        drop(Box::from_raw(ptr.cast::<InnerRuntime>()));
    }
}

pub(crate) struct InnerRuntime {
    pub runs: Mutex<HashMap<String, InnerRun>>,
    pub tools: Mutex<ToolRegistry>,
    pub journal: Arc<dyn ancora_core::journal::JournalStore>,
    pub model_backend: ModelBackend,
}

impl Default for InnerRuntime {
    fn default() -> Self {
        Self::with_model_backend(ModelBackend::Offline)
    }
}

impl InnerRuntime {
    pub fn with_model_backend(model_backend: ModelBackend) -> Self {
        Self {
            runs: Mutex::new(HashMap::new()),
            tools: Mutex::new(ToolRegistry::new()),
            journal: Arc::new(ancora_core::journal::MemoryStore::new()),
            model_backend,
        }
    }
}
