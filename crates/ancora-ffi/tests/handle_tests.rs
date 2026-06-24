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
    assert_ne!(AncorErrorCode::Ok as i32, AncorErrorCode::InvalidUtf8 as i32);
    assert_ne!(AncorErrorCode::Ok as i32, AncorErrorCode::Internal as i32);
}

#[test]
fn buffer_new_and_free_does_not_leak() {
    let data = b"hello world";
    let buf = ancora_buffer_new(data.as_ptr(), data.len());
    assert_eq!(buf.len, data.len());
    assert!(!buf.ptr.is_null());
    ancora_buffer_free(buf);
}

#[test]
fn buffer_free_null_is_noop() {
    use ancora_ffi::buffer::AncorBuffer;
    let empty = AncorBuffer { ptr: std::ptr::null_mut(), len: 0 };
    ancora_buffer_free(empty);
}
