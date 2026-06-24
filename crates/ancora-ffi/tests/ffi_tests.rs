use ancora_ffi::runtime::{ancora_create_runtime, ancora_free_runtime};
use ancora_ffi::run_id::{ancora_run_id_free, ancora_run_id_new};
use ancora_ffi::version::ancora_version;

#[test]
fn ancora_version_returns_non_null() {
    let ptr = ancora_version();
    assert!(!ptr.is_null());
}

#[test]
fn ancora_version_is_valid_utf8() {
    let ptr = ancora_version();
    assert!(!ptr.is_null());
    let s = unsafe { std::ffi::CStr::from_ptr(ptr) }.to_str().unwrap();
    assert!(!s.is_empty());
}

#[test]
fn create_and_free_runtime_does_not_panic() {
    let rt = ancora_create_runtime();
    assert!(!rt.is_null());
    ancora_free_runtime(rt);
}

#[test]
fn free_null_runtime_is_noop() {
    ancora_free_runtime(std::ptr::null_mut());
}
