use ancora_ffi::buffer::{ancora_buffer_free, ancora_buffer_from_str, AncorBuffer};
use ancora_ffi::error_code::AncorErrorCode;
use ancora_ffi::runtime::{ancora_free_runtime, ancora_runtime_new};
use ancora_ffi::tool_ops::{
    ancora_tool_count, ancora_tool_exists, ancora_tool_invoke, ancora_tool_register,
    ancora_tool_unregister,
};

fn make_rt() -> *mut ancora_ffi::handles::AncorRuntime {
    let mut rt = std::ptr::null_mut();
    ancora_runtime_new(&mut rt);
    rt
}

fn cstr(s: &str) -> std::ffi::CString {
    std::ffi::CString::new(s).unwrap()
}

unsafe extern "C" fn echo_cb(
    input: *const u8,
    input_len: usize,
    out: *mut AncorBuffer,
) -> AncorErrorCode {
    let slice = std::slice::from_raw_parts(input, input_len);
    let s = std::str::from_utf8_unchecked(slice);
    *out = ancora_buffer_from_str(s);
    AncorErrorCode::Ok
}

#[test]
fn register_tool_count_is_one() {
    let rt = make_rt();
    let name = cstr("echo");
    let code = ancora_tool_register(rt, name.as_ptr(), echo_cb);
    assert_eq!(code, AncorErrorCode::Ok);
    assert_eq!(ancora_tool_count(rt), 1);
    ancora_free_runtime(rt);
}

#[test]
fn tool_exists_returns_one_after_register() {
    let rt = make_rt();
    let name = cstr("echo");
    ancora_tool_register(rt, name.as_ptr(), echo_cb);
    assert_eq!(ancora_tool_exists(rt, name.as_ptr()), 1);
    ancora_free_runtime(rt);
}
