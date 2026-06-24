use ancora_ffi::runtime::{ancora_create_runtime, ancora_free_runtime};
use ancora_ffi::run_id::{ancora_run_id_free, ancora_run_id_new};
use ancora_ffi::version::ancora_version;

#[test]
fn ancora_version_returns_non_null() {
    let ptr = ancora_version();
    assert!(!ptr.is_null());
}
