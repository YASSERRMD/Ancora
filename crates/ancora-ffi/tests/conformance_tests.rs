use ancora_ffi::buffer::{ancora_buffer_free, AncorBuffer};
use ancora_ffi::error_code::AncorErrorCode;
use ancora_ffi::run_ops::{ancora_run_poll, ancora_run_resume, ancora_run_start};
use ancora_ffi::runtime::{ancora_free_runtime, ancora_runtime_new};

fn make_rt() -> *mut ancora_ffi::handles::AncorRuntime {
    let mut rt = std::ptr::null_mut();
    ancora_runtime_new(&mut rt);
    rt
}

fn start_run(rt: *mut ancora_ffi::handles::AncorRuntime) -> String {
    let spec = b"{}";
    let mut out = AncorBuffer { ptr: std::ptr::null_mut(), len: 0 };
    ancora_run_start(rt, spec.as_ptr(), spec.len(), &mut out);
    let slice = unsafe { std::slice::from_raw_parts(out.ptr, out.len) };
    let id = String::from_utf8_lossy(slice).into_owned();
    ancora_buffer_free(out);
    id
}

fn drain_events(rt: *mut ancora_ffi::handles::AncorRuntime, run_id: &str) -> Vec<String> {
    let c_id = std::ffi::CString::new(run_id).unwrap();
    let mut events = Vec::new();
    loop {
        let mut ev = AncorBuffer { ptr: std::ptr::null_mut(), len: 0 };
        ancora_run_poll(rt, c_id.as_ptr(), &mut ev);
        if ev.ptr.is_null() || ev.len == 0 {
            break;
        }
        let s = unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(ev.ptr, ev.len)) }.to_owned();
        ancora_buffer_free(ev);
        events.push(s);
    }
    events
}

#[test]
fn single_agent_scenario_start_returns_ok() {
    let rt = make_rt();
    let spec = b"{}";
    let mut out = AncorBuffer { ptr: std::ptr::null_mut(), len: 0 };
    let code = ancora_run_start(rt, spec.as_ptr(), spec.len(), &mut out);
    assert_eq!(code, AncorErrorCode::Ok, "single-agent: start must return Ok");
    ancora_buffer_free(out);
    ancora_free_runtime(rt);
}

#[test]
fn single_agent_scenario_produces_started_and_completed_events() {
    let rt = make_rt();
    let id = start_run(rt);
    let events = drain_events(rt, &id);
    assert!(events.iter().any(|e| e.contains("started")), "single-agent: missing started event, got: {events:?}");
    assert!(events.iter().any(|e| e.contains("completed")), "single-agent: missing completed event, got: {events:?}");
    ancora_free_runtime(rt);
}
