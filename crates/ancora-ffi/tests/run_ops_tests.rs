use ancora_ffi::buffer::{ancora_buffer_free, AncorBuffer};
use ancora_ffi::error_code::AncorErrorCode;
use ancora_ffi::run_ops::{ancora_run_cost, ancora_run_poll, ancora_run_resume, ancora_run_start};
use ancora_ffi::runtime::{ancora_free_runtime, ancora_runtime_new};

fn make_rt() -> *mut ancora_ffi::handles::AncorRuntime {
    let mut rt = std::ptr::null_mut();
    ancora_runtime_new(&mut rt);
    rt
}

fn buf_to_string(buf: &AncorBuffer) -> String {
    if buf.ptr.is_null() || buf.len == 0 {
        return String::new();
    }
    let slice = unsafe { std::slice::from_raw_parts(buf.ptr, buf.len) };
    String::from_utf8_lossy(slice).into_owned()
}

fn cstr(s: &str) -> std::ffi::CString {
    std::ffi::CString::new(s).unwrap()
}
