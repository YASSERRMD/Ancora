use ancora_ffi::buffer::{ancora_buffer_free, ancora_buffer_from_str, ancora_buffer_new};
use ancora_ffi::error_code::AncorErrorCode;
use ancora_ffi::run_id::{ancora_run_id_free, ancora_run_id_new, ancora_run_id_to_str};
use ancora_ffi::runtime::{ancora_free_runtime, ancora_runtime_new};

#[test]
fn error_code_ok_is_zero() {
    assert_eq!(AncorErrorCode::Ok as i32, 0);
}

#[test]
fn error_codes_are_distinct() {
    assert_ne!(AncorErrorCode::Ok as i32, AncorErrorCode::NullPtr as i32);
    assert_ne!(
        AncorErrorCode::Ok as i32,
        AncorErrorCode::InvalidUtf8 as i32
    );
    assert_ne!(AncorErrorCode::Ok as i32, AncorErrorCode::Internal as i32);
}

#[test]
fn buffer_new_and_free_does_not_leak() {
    let data = b"hello world";
    let buf = unsafe { ancora_buffer_new(data.as_ptr(), data.len()) };
    assert_eq!(buf.len, data.len());
    assert!(!buf.ptr.is_null());
    unsafe { ancora_buffer_free(buf) };
}

#[test]
fn buffer_free_null_is_noop() {
    use ancora_ffi::buffer::AncorBuffer;
    let empty = AncorBuffer {
        ptr: std::ptr::null_mut(),
        len: 0,
    };
    unsafe { ancora_buffer_free(empty) };
}

#[test]
fn buffer_from_str_roundtrips() {
    let buf = ancora_buffer_from_str("ancora-test");
    assert_eq!(buf.len, "ancora-test".len());
    let slice = unsafe { std::slice::from_raw_parts(buf.ptr, buf.len) };
    assert_eq!(slice, b"ancora-test");
    unsafe { ancora_buffer_free(buf) };
}

#[test]
fn runtime_new_via_error_code_api_returns_ok() {
    let mut rt = std::ptr::null_mut();
    let code = unsafe { ancora_runtime_new(&mut rt) };
    assert_eq!(code, AncorErrorCode::Ok);
    assert!(!rt.is_null());
    unsafe { ancora_free_runtime(rt) };
}

#[test]
fn runtime_new_null_out_returns_null_ptr_error() {
    let code = unsafe { ancora_runtime_new(std::ptr::null_mut()) };
    assert_eq!(code, AncorErrorCode::NullPtr);
}

#[test]
fn run_id_to_str_round_trips() {
    let s = std::ffi::CString::new("my-run-id").unwrap();
    let id = unsafe { ancora_run_id_new(s.as_ptr()) };
    assert!(!id.is_null());
    let buf = unsafe { ancora_run_id_to_str(id.cast_const()) };
    assert_eq!(buf.len, "my-run-id".len());
    unsafe { ancora_buffer_free(buf) };
    unsafe { ancora_run_id_free(id) };
}
