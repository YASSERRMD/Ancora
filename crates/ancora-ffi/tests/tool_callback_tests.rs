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

#[test]
fn tool_exists_returns_zero_for_unknown() {
    let rt = make_rt();
    let name = cstr("unknown");
    assert_eq!(ancora_tool_exists(rt, name.as_ptr()), 0);
    ancora_free_runtime(rt);
}

#[test]
fn unregister_tool_count_drops_to_zero() {
    let rt = make_rt();
    let name = cstr("echo");
    ancora_tool_register(rt, name.as_ptr(), echo_cb);
    assert_eq!(ancora_tool_count(rt), 1);
    ancora_tool_unregister(rt, name.as_ptr());
    assert_eq!(ancora_tool_count(rt), 0);
    ancora_free_runtime(rt);
}

#[test]
fn invoke_echo_tool_returns_input() {
    let rt = make_rt();
    let name = cstr("echo");
    ancora_tool_register(rt, name.as_ptr(), echo_cb);
    let input = b"hello";
    let mut out = AncorBuffer { ptr: std::ptr::null_mut(), len: 0 };
    let code = ancora_tool_invoke(rt, name.as_ptr(), input.as_ptr(), input.len(), &mut out);
    assert_eq!(code, AncorErrorCode::Ok);
    let slice = unsafe { std::slice::from_raw_parts(out.ptr, out.len) };
    assert_eq!(slice, b"hello");
    ancora_buffer_free(out);
    ancora_free_runtime(rt);
}
