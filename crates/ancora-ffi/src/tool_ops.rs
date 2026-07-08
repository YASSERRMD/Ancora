use std::ffi::CStr;
use std::os::raw::c_char;

use crate::buffer::AncorBuffer;
use crate::error_code::AncorErrorCode;
use crate::handles::AncorRuntime;
use crate::runtime::InnerRuntime;
use crate::tool_callback::AncorToolCallback;

/// Register a named tool callback on the runtime.
/// Returns `NullPtr` if either `rt` or `name` is null.
///
/// # Safety
/// `rt` must be a live runtime pointer. `name` must be a valid
/// null-terminated C string. `cb` must be safe to call with a byte buffer
/// for as long as it remains registered.
#[no_mangle]
pub unsafe extern "C" fn ancora_tool_register(
    rt: *mut AncorRuntime,
    name: *const c_char,
    cb: AncorToolCallback,
) -> AncorErrorCode {
    if rt.is_null() || name.is_null() {
        return AncorErrorCode::NullPtr;
    }
    let inner = unsafe { &*(rt.cast::<InnerRuntime>()) };
    let name_str = match unsafe { CStr::from_ptr(name) }.to_str() {
        Ok(s) => s.to_owned(),
        Err(_) => return AncorErrorCode::InvalidUtf8,
    };
    inner.tools.lock().unwrap().register(name_str, cb);
    AncorErrorCode::Ok
}

/// Unregister a named tool callback. Returns `NullPtr` if either pointer is null.
///
/// # Safety
/// `rt` must be a live runtime pointer. `name` must be a valid
/// null-terminated C string.
#[no_mangle]
pub unsafe extern "C" fn ancora_tool_unregister(
    rt: *mut AncorRuntime,
    name: *const c_char,
) -> AncorErrorCode {
    if rt.is_null() || name.is_null() {
        return AncorErrorCode::NullPtr;
    }
    let inner = unsafe { &*(rt.cast::<InnerRuntime>()) };
    let name_str = match unsafe { CStr::from_ptr(name) }.to_str() {
        Ok(s) => s,
        Err(_) => return AncorErrorCode::InvalidUtf8,
    };
    inner.tools.lock().unwrap().unregister(name_str);
    AncorErrorCode::Ok
}

/// Invoke a named tool with `input_bytes` and write the output into `out`.
/// Returns `NullPtr` if any required pointer is null, `Internal` if the tool is not found.
///
/// # Safety
/// `rt` must be a live runtime pointer. `name` must be a valid
/// null-terminated C string. If `input_bytes` is non-null it must point to
/// at least `input_len` valid bytes. `out` must point to valid, writable
/// memory for an `AncorBuffer`.
#[no_mangle]
pub unsafe extern "C" fn ancora_tool_invoke(
    rt: *mut AncorRuntime,
    name: *const c_char,
    input_bytes: *const u8,
    input_len: usize,
    out: *mut AncorBuffer,
) -> AncorErrorCode {
    if rt.is_null() || name.is_null() || out.is_null() {
        return AncorErrorCode::NullPtr;
    }
    let inner = unsafe { &*(rt.cast::<InnerRuntime>()) };
    let name_str = match unsafe { CStr::from_ptr(name) }.to_str() {
        Ok(s) => s,
        Err(_) => return AncorErrorCode::InvalidUtf8,
    };
    let cb = match inner.tools.lock().unwrap().get(name_str) {
        Some(f) => f,
        None => return AncorErrorCode::Internal,
    };
    unsafe { cb(input_bytes, input_len, out) }
}

/// Return the number of registered tools. Returns 0 if `rt` is null.
///
/// # Safety
/// If `rt` is non-null, it must be a live runtime pointer.
#[no_mangle]
pub unsafe extern "C" fn ancora_tool_count(rt: *mut AncorRuntime) -> usize {
    if rt.is_null() {
        return 0;
    }
    let inner = unsafe { &*(rt.cast::<InnerRuntime>()) };
    inner.tools.lock().unwrap().count()
}

/// Return 1 if a tool with `name` is registered, 0 otherwise. Returns 0 if any pointer is null.
///
/// # Safety
/// If `rt` is non-null, it must be a live runtime pointer. If `name` is
/// non-null, it must be a valid null-terminated C string.
#[no_mangle]
pub unsafe extern "C" fn ancora_tool_exists(rt: *mut AncorRuntime, name: *const c_char) -> u8 {
    if rt.is_null() || name.is_null() {
        return 0;
    }
    let inner = unsafe { &*(rt.cast::<InnerRuntime>()) };
    let name_str = match unsafe { CStr::from_ptr(name) }.to_str() {
        Ok(s) => s,
        Err(_) => return 0,
    };
    if inner.tools.lock().unwrap().contains(name_str) {
        1
    } else {
        0
    }
}
