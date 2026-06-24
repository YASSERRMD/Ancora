use std::ffi::CStr;
use std::os::raw::c_char;

use crate::buffer::AncorBuffer;
use crate::error_code::AncorErrorCode;
use crate::handles::AncorRuntime;
use crate::runtime::InnerRuntime;
use crate::tool_callback::AncorToolCallback;

/// Register a named tool callback on the runtime.
/// Returns `NullPtr` if either `rt` or `name` is null.
#[no_mangle]
pub extern "C" fn ancora_tool_register(
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
