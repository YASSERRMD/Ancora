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

#[test]
fn run_start_returns_non_empty_run_id() {
    let rt = make_rt();
    let spec = b"{}";
    let mut out = AncorBuffer { ptr: std::ptr::null_mut(), len: 0 };
    let code = ancora_run_start(rt, spec.as_ptr(), spec.len(), &mut out);
    assert_eq!(code, AncorErrorCode::Ok);
    assert!(out.len > 0, "run id buffer should not be empty");
    ancora_buffer_free(out);
    ancora_free_runtime(rt);
}

fn start_run(rt: *mut ancora_ffi::handles::AncorRuntime) -> String {
    let spec = b"{}";
    let mut out = AncorBuffer { ptr: std::ptr::null_mut(), len: 0 };
    ancora_run_start(rt, spec.as_ptr(), spec.len(), &mut out);
    let id = buf_to_string(&out);
    ancora_buffer_free(out);
    id
}

#[test]
fn run_poll_first_event_is_started() {
    let rt = make_rt();
    let id = start_run(rt);
    let c_id = cstr(&id);
    let mut event = AncorBuffer { ptr: std::ptr::null_mut(), len: 0 };
    ancora_run_poll(rt, c_id.as_ptr(), &mut event);
    let s = buf_to_string(&event);
    assert!(s.contains("started"), "first event should be started, got: {s}");
    ancora_buffer_free(event);
    ancora_free_runtime(rt);
}

#[test]
fn run_poll_second_event_is_completed() {
    let rt = make_rt();
    let id = start_run(rt);
    let c_id = cstr(&id);
    let mut e1 = AncorBuffer { ptr: std::ptr::null_mut(), len: 0 };
    let mut e2 = AncorBuffer { ptr: std::ptr::null_mut(), len: 0 };
    ancora_run_poll(rt, c_id.as_ptr(), &mut e1);
    ancora_run_poll(rt, c_id.as_ptr(), &mut e2);
    let s = buf_to_string(&e2);
    assert!(s.contains("completed"), "second event should be completed, got: {s}");
    ancora_buffer_free(e1);
    ancora_buffer_free(e2);
    ancora_free_runtime(rt);
}

#[test]
fn run_poll_returns_empty_after_all_events_consumed() {
    let rt = make_rt();
    let id = start_run(rt);
    let c_id = cstr(&id);
    let empty = AncorBuffer { ptr: std::ptr::null_mut(), len: 0 };
    let mut e1 = empty;
    let mut e2 = empty;
    let mut e3 = empty;
    ancora_run_poll(rt, c_id.as_ptr(), &mut e1);
    ancora_run_poll(rt, c_id.as_ptr(), &mut e2);
    ancora_run_poll(rt, c_id.as_ptr(), &mut e3);
    assert!(e3.ptr.is_null() && e3.len == 0, "third poll should be empty");
    ancora_buffer_free(e1);
    ancora_buffer_free(e2);
    ancora_free_runtime(rt);
}

#[test]
fn run_cost_returns_json_with_total_usd() {
    let rt = make_rt();
    let id = start_run(rt);
    let c_id = cstr(&id);
    let mut cost = AncorBuffer { ptr: std::ptr::null_mut(), len: 0 };
    let code = ancora_run_cost(rt, c_id.as_ptr(), &mut cost);
    assert_eq!(code, AncorErrorCode::Ok);
    let s = buf_to_string(&cost);
    assert!(s.contains("total_usd"), "cost should contain total_usd, got: {s}");
    ancora_buffer_free(cost);
    ancora_free_runtime(rt);
}
