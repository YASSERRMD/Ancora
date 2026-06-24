use ancora_ffi::buffer::{ancora_buffer_free, ancora_buffer_from_str, ancora_buffer_new};
use ancora_ffi::error_code::AncorErrorCode;
use ancora_ffi::run_id::{ancora_run_id_free, ancora_run_id_new, ancora_run_id_to_str};
use ancora_ffi::runtime::{ancora_free_runtime, ancora_runtime_new};

#[test]
fn error_code_ok_is_zero() {
    assert_eq!(AncorErrorCode::Ok as i32, 0);
}
