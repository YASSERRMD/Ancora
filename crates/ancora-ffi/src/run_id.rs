use std::os::raw::c_char;

use crate::buffer::{ancora_buffer_from_str, AncorBuffer};
use crate::handles::AncorRunId;

struct InnerRunId {
    id: String,
}

/// Allocate a new run ID from a null-terminated UTF-8 string.
/// Returns null if `s` is null or not valid UTF-8.
#[no_mangle]
pub extern "C" fn ancora_run_id_new(s: *const c_char) -> *mut AncorRunId {
    if s.is_null() {
        return std::ptr::null_mut();
    }
    let cstr = unsafe { std::ffi::CStr::from_ptr(s) };
    match cstr.to_str() {
        Ok(id) => {
            let boxed = Box::new(InnerRunId { id: id.to_string() });
            Box::into_raw(boxed).cast()
        }
        Err(_) => std::ptr::null_mut(),
    }
}

/// Free a run ID previously created by `ancora_run_id_new`.
/// Passing null is a no-op.
#[no_mangle]
pub extern "C" fn ancora_run_id_free(ptr: *mut AncorRunId) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        drop(Box::from_raw(ptr.cast::<InnerRunId>()));
    }
}

/// Return the run ID string as an owned `AncorBuffer`.
/// The buffer must be freed with `ancora_buffer_free`.
/// Returns a zero-length buffer if `ptr` is null.
#[no_mangle]
pub extern "C" fn ancora_run_id_to_str(ptr: *const AncorRunId) -> AncorBuffer {
    if ptr.is_null() {
        return AncorBuffer { ptr: std::ptr::null_mut(), len: 0 };
    }
    let inner = unsafe { &*(ptr.cast::<InnerRunId>()) };
    ancora_buffer_from_str(&inner.id)
}
