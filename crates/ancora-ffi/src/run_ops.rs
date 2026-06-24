use std::os::raw::c_char;

use crate::buffer::{ancora_buffer_from_str, AncorBuffer};
use crate::error_code::AncorErrorCode;
use crate::handles::AncorRuntime;
use crate::runs::InnerRun;
use crate::runtime::InnerRuntime;

/// Start a new run from serialized agent spec bytes.
/// Writes the run ID (as UTF-8) into `out_run_id`.
/// Returns `NullPtr` if runtime or spec pointer is null.
#[no_mangle]
pub extern "C" fn ancora_run_start(
    rt: *mut AncorRuntime,
    spec_bytes: *const u8,
    spec_len: usize,
    out_run_id: *mut AncorBuffer,
) -> AncorErrorCode {
    if rt.is_null() || spec_bytes.is_null() || out_run_id.is_null() {
        return AncorErrorCode::NullPtr;
    }
    let spec_str = if spec_len == 0 {
        String::new()
    } else {
        let slice = unsafe { std::slice::from_raw_parts(spec_bytes, spec_len) };
        String::from_utf8_lossy(slice).into_owned()
    };
    let run_id = uuid::Uuid::new_v4().to_string();
    let run = InnerRun::new(&run_id, &spec_str);
    let inner = unsafe { &mut *rt.cast::<InnerRuntime>() };
    inner.runs.lock().unwrap().insert(run_id.clone(), run);
    unsafe { *out_run_id = ancora_buffer_from_str(&run_id) };
    AncorErrorCode::Ok
}

/// Poll the next event for a run. Writes event JSON bytes into `out_event`.
/// Returns an empty buffer in `out_event` when all events are consumed.
/// Returns `NullPtr` if any pointer is null.
#[no_mangle]
pub extern "C" fn ancora_run_poll(
    rt: *mut AncorRuntime,
    run_id: *const c_char,
    out_event: *mut AncorBuffer,
) -> AncorErrorCode {
    if rt.is_null() || run_id.is_null() || out_event.is_null() {
        return AncorErrorCode::NullPtr;
    }
    let id = unsafe { std::ffi::CStr::from_ptr(run_id) }
        .to_str()
        .unwrap_or("");
    let inner = unsafe { &mut *rt.cast::<InnerRuntime>() };
    let mut guard = inner.runs.lock().unwrap();
    if let Some(run) = guard.get_mut(id) {
        if let Some(event) = run.poll_event() {
            unsafe { *out_event = ancora_buffer_from_str(&event) };
        } else {
            unsafe { *out_event = AncorBuffer { ptr: std::ptr::null_mut(), len: 0 } };
        }
    } else {
        unsafe { *out_event = AncorBuffer { ptr: std::ptr::null_mut(), len: 0 } };
    }
    AncorErrorCode::Ok
}
