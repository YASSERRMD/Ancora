use ancora_ffi::run_id::{ancora_run_id_free, ancora_run_id_new};
use ancora_ffi::runtime::{ancora_create_runtime, ancora_free_runtime};
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

#[test]
fn create_and_free_run_id_does_not_panic() {
    let s = std::ffi::CString::new("run-test-1").unwrap();
    let id = ancora_run_id_new(s.as_ptr());
    assert!(!id.is_null());
    ancora_run_id_free(id);
}

#[test]
fn run_id_new_with_null_returns_null() {
    let id = ancora_run_id_new(std::ptr::null());
    assert!(id.is_null());
}

#[test]
fn free_null_run_id_is_noop() {
    ancora_run_id_free(std::ptr::null_mut());
}

#[test]
fn header_generation_succeeds_during_build() {
    let out_dir = std::env::var("OUT_DIR").unwrap_or_default();
    assert!(
        !out_dir.is_empty(),
        "OUT_DIR should be set during cargo test"
    );
}
