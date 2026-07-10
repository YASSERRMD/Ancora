use ancora_ffi::buffer::{ancora_buffer_free, AncorBuffer};
use ancora_ffi::error_code::AncorErrorCode;
use ancora_ffi::run_ops::{ancora_run_cost, ancora_run_poll, ancora_run_resume, ancora_run_start};
use ancora_ffi::runtime::{ancora_free_runtime, ancora_runtime_new};

fn make_rt() -> *mut ancora_ffi::handles::AncorRuntime {
    let mut rt = std::ptr::null_mut();
    unsafe { ancora_runtime_new(&mut rt) };
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
    let mut out = AncorBuffer {
        ptr: std::ptr::null_mut(),
        len: 0,
    };
    let code = unsafe { ancora_run_start(rt, spec.as_ptr(), spec.len(), &mut out) };
    assert_eq!(code, AncorErrorCode::Ok);
    assert!(out.len > 0, "run id buffer should not be empty");
    unsafe { ancora_buffer_free(out) };
    unsafe { ancora_free_runtime(rt) };
}

fn start_run(rt: *mut ancora_ffi::handles::AncorRuntime) -> String {
    let spec = b"{}";
    let mut out = AncorBuffer {
        ptr: std::ptr::null_mut(),
        len: 0,
    };
    unsafe { ancora_run_start(rt, spec.as_ptr(), spec.len(), &mut out) };
    let id = buf_to_string(&out);
    unsafe { ancora_buffer_free(out) };
    id
}

#[test]
fn run_poll_first_event_is_started() {
    let rt = make_rt();
    let id = start_run(rt);
    let c_id = cstr(&id);
    let mut event = AncorBuffer {
        ptr: std::ptr::null_mut(),
        len: 0,
    };
    unsafe { ancora_run_poll(rt, c_id.as_ptr(), &mut event) };
    let s = buf_to_string(&event);
    assert!(
        s.contains("started"),
        "first event should be started, got: {s}"
    );
    unsafe { ancora_buffer_free(event) };
    unsafe { ancora_free_runtime(rt) };
}

#[test]
fn run_poll_second_event_is_completed() {
    let rt = make_rt();
    let id = start_run(rt);
    let c_id = cstr(&id);
    let mut e1 = AncorBuffer {
        ptr: std::ptr::null_mut(),
        len: 0,
    };
    let mut e2 = AncorBuffer {
        ptr: std::ptr::null_mut(),
        len: 0,
    };
    unsafe { ancora_run_poll(rt, c_id.as_ptr(), &mut e1) };
    unsafe { ancora_run_poll(rt, c_id.as_ptr(), &mut e2) };
    let s = buf_to_string(&e2);
    assert!(
        s.contains("completed"),
        "second event should be completed, got: {s}"
    );
    unsafe { ancora_buffer_free(e1) };
    unsafe { ancora_buffer_free(e2) };
    unsafe { ancora_free_runtime(rt) };
}

#[test]
fn run_poll_returns_empty_after_all_events_consumed() {
    let rt = make_rt();
    let id = start_run(rt);
    let c_id = cstr(&id);
    let empty = AncorBuffer {
        ptr: std::ptr::null_mut(),
        len: 0,
    };
    let mut e1 = empty;
    let mut e2 = empty;
    let mut e3 = empty;
    unsafe { ancora_run_poll(rt, c_id.as_ptr(), &mut e1) };
    unsafe { ancora_run_poll(rt, c_id.as_ptr(), &mut e2) };
    unsafe { ancora_run_poll(rt, c_id.as_ptr(), &mut e3) };
    assert!(
        e3.ptr.is_null() && e3.len == 0,
        "third poll should be empty"
    );
    unsafe { ancora_buffer_free(e1) };
    unsafe { ancora_buffer_free(e2) };
    unsafe { ancora_free_runtime(rt) };
}

#[test]
fn run_cost_returns_json_with_total_usd() {
    let rt = make_rt();
    let id = start_run(rt);
    let c_id = cstr(&id);
    let mut cost = AncorBuffer {
        ptr: std::ptr::null_mut(),
        len: 0,
    };
    let code = unsafe { ancora_run_cost(rt, c_id.as_ptr(), &mut cost) };
    assert_eq!(code, AncorErrorCode::Ok);
    let s = buf_to_string(&cost);
    assert!(
        s.contains("total_usd"),
        "cost should contain total_usd, got: {s}"
    );
    unsafe { ancora_buffer_free(cost) };
    unsafe { ancora_free_runtime(rt) };
}

#[test]
fn run_resume_returns_ok() {
    let rt = make_rt();
    let id = start_run(rt);
    let c_id = cstr(&id);
    let decision = b"approved";
    let code = unsafe { ancora_run_resume(rt, c_id.as_ptr(), decision.as_ptr(), decision.len()) };
    assert_eq!(code, AncorErrorCode::Ok);
    unsafe { ancora_free_runtime(rt) };
}

#[test]
fn run_start_with_null_runtime_returns_null_ptr() {
    let spec = b"{}";
    let mut out = AncorBuffer {
        ptr: std::ptr::null_mut(),
        len: 0,
    };
    let code =
        unsafe { ancora_run_start(std::ptr::null_mut(), spec.as_ptr(), spec.len(), &mut out) };
    assert_eq!(code, AncorErrorCode::NullPtr);
}

#[test]
fn resume_after_poll_adds_resumed_event() {
    let rt = make_rt();
    let id = start_run(rt);
    let c_id = cstr(&id);
    let zero = AncorBuffer {
        ptr: std::ptr::null_mut(),
        len: 0,
    };
    let mut e1 = zero;
    let mut e2 = zero;
    unsafe { ancora_run_poll(rt, c_id.as_ptr(), &mut e1) };
    unsafe { ancora_run_poll(rt, c_id.as_ptr(), &mut e2) };
    let decision = b"approved";
    unsafe { ancora_run_resume(rt, c_id.as_ptr(), decision.as_ptr(), decision.len()) };
    let mut e3 = zero;
    unsafe { ancora_run_poll(rt, c_id.as_ptr(), &mut e3) };
    let s3 = buf_to_string(&e3);
    assert!(s3.contains("resumed"), "expected resumed event, got: {s3}");
    unsafe { ancora_buffer_free(e1) };
    unsafe { ancora_buffer_free(e2) };
    unsafe { ancora_buffer_free(e3) };
    unsafe { ancora_free_runtime(rt) };
}

#[test]
fn drive_full_run_start_poll_poll_cost() {
    let rt = make_rt();
    let id = start_run(rt);
    let c_id = cstr(&id);
    let zero = AncorBuffer {
        ptr: std::ptr::null_mut(),
        len: 0,
    };
    let mut e1 = zero;
    let mut e2 = zero;
    let mut cost = zero;
    unsafe { ancora_run_poll(rt, c_id.as_ptr(), &mut e1) };
    unsafe { ancora_run_poll(rt, c_id.as_ptr(), &mut e2) };
    unsafe { ancora_run_cost(rt, c_id.as_ptr(), &mut cost) };
    let s1 = buf_to_string(&e1);
    let s2 = buf_to_string(&e2);
    let sc = buf_to_string(&cost);
    assert!(s1.contains("started"), "e1={s1}");
    assert!(s2.contains("completed"), "e2={s2}");
    assert!(sc.contains("total_usd"), "cost={sc}");
    unsafe { ancora_buffer_free(e1) };
    unsafe { ancora_buffer_free(e2) };
    unsafe { ancora_buffer_free(cost) };
    unsafe { ancora_free_runtime(rt) };
}

/// Proves the run engine is real, not the old hard-coded stub: the
/// `completed` event's output must equal what the offline echo model client
/// actually produced from the spec's instructions, not a fixed string.
#[test]
fn completed_event_output_reflects_real_agent_execution() {
    let rt = make_rt();
    let spec = br#"{"model_id":"mock","instructions":"echo this exact phrase"}"#;
    let mut out = AncorBuffer {
        ptr: std::ptr::null_mut(),
        len: 0,
    };
    unsafe { ancora_run_start(rt, spec.as_ptr(), spec.len(), &mut out) };
    let id = buf_to_string(&out);
    unsafe { ancora_buffer_free(out) };
    let c_id = cstr(&id);

    let mut started = AncorBuffer {
        ptr: std::ptr::null_mut(),
        len: 0,
    };
    let mut completed = AncorBuffer {
        ptr: std::ptr::null_mut(),
        len: 0,
    };
    unsafe { ancora_run_poll(rt, c_id.as_ptr(), &mut started) };
    unsafe { ancora_run_poll(rt, c_id.as_ptr(), &mut completed) };
    let completed_json = buf_to_string(&completed);
    assert!(
        completed_json.contains("echo this exact phrase"),
        "completed event should carry the real echoed output, got: {completed_json}"
    );
    unsafe { ancora_buffer_free(started) };
    unsafe { ancora_buffer_free(completed) };
    unsafe { ancora_free_runtime(rt) };
}

/// A provider-configured runtime pointed at an unreachable endpoint must
/// fail the run gracefully (a `failed` event), not panic or hang.
#[test]
fn provider_backend_pointed_at_unreachable_host_produces_failed_event() {
    use ancora_ffi::runtime::ancora_runtime_new_with_config;

    let config =
        br#"{"provider":{"base_url":"http://127.0.0.1:1","auth_env_var":"UNSET_TEST_KEY"}}"#;
    let mut rt = std::ptr::null_mut();
    unsafe { ancora_runtime_new_with_config(config.as_ptr(), config.len(), &mut rt) };

    let spec = br#"{"model_id":"mock","instructions":"hello"}"#;
    let mut out = AncorBuffer {
        ptr: std::ptr::null_mut(),
        len: 0,
    };
    unsafe { ancora_run_start(rt, spec.as_ptr(), spec.len(), &mut out) };
    let id = buf_to_string(&out);
    unsafe { ancora_buffer_free(out) };
    let c_id = cstr(&id);

    let mut started = AncorBuffer {
        ptr: std::ptr::null_mut(),
        len: 0,
    };
    let mut failed = AncorBuffer {
        ptr: std::ptr::null_mut(),
        len: 0,
    };
    unsafe { ancora_run_poll(rt, c_id.as_ptr(), &mut started) };
    unsafe { ancora_run_poll(rt, c_id.as_ptr(), &mut failed) };
    let failed_json = buf_to_string(&failed);
    assert!(
        failed_json.contains("failed"),
        "unreachable provider should produce a failed event, got: {failed_json}"
    );
    unsafe { ancora_buffer_free(started) };
    unsafe { ancora_buffer_free(failed) };
    unsafe { ancora_free_runtime(rt) };
}
