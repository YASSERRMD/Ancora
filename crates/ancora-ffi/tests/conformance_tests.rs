use ancora_core::conformance;
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

fn event_kind(event: &str) -> &str {
    if event.contains("started") { "started" }
    else if event.contains("completed") { "completed" }
    else if event.contains("resumed") { "resumed" }
    else { event }
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

#[test]
fn single_agent_scenario_run_id_is_nonempty() {
    let rt = make_rt();
    let id = start_run(rt);
    assert!(!id.is_empty(), "single-agent: run id must be non-empty");
    ancora_free_runtime(rt);
}

#[test]
fn multi_agent_verifier_scenario_two_runs_have_different_ids() {
    let rt = make_rt();
    let id1 = start_run(rt);
    let id2 = start_run(rt);
    assert_ne!(id1, id2, "multi-agent-verifier: each run must have a unique id");
    ancora_free_runtime(rt);
}

#[test]
fn human_in_loop_scenario_resume_produces_resumed_event() {
    let rt = make_rt();
    let id = start_run(rt);
    let c_id = std::ffi::CString::new(id.as_str()).unwrap();
    drain_events(rt, &id);
    let decision = b"approved";
    let code = ancora_run_resume(rt, c_id.as_ptr(), decision.as_ptr(), decision.len());
    assert_eq!(code, AncorErrorCode::Ok, "human-in-loop: resume must return Ok");
    let events = drain_events(rt, &id);
    assert!(events.iter().any(|e| e.contains("resumed")), "human-in-loop: missing resumed event, got: {events:?}");
    ancora_free_runtime(rt);
}

#[test]
fn crash_and_recover_scenario_events_are_deterministic() {
    let rt1 = make_rt();
    let id1 = start_run(rt1);
    let events1 = drain_events(rt1, &id1);
    ancora_free_runtime(rt1);

    let rt2 = make_rt();
    let id2 = start_run(rt2);
    let events2 = drain_events(rt2, &id2);
    ancora_free_runtime(rt2);

    let kinds1: Vec<_> = events1.iter().map(|e| event_kind(e)).collect();
    let kinds2: Vec<_> = events2.iter().map(|e| event_kind(e)).collect();
    assert_eq!(kinds1, kinds2, "crash-and-recover: event kinds must be deterministic across runs");
}

#[test]
fn all_conformance_scenarios_have_nonempty_ids() {
    for scenario in conformance::all_scenarios() {
        assert!(!scenario.id.is_empty(), "scenario id must be non-empty");
    }
}

#[test]
fn all_conformance_scenarios_have_nonempty_descriptions() {
    for scenario in conformance::all_scenarios() {
        assert!(!scenario.description.is_empty(), "scenario description must be non-empty");
    }
}

#[test]
fn journal_single_agent_event_order_is_started_then_completed() {
    let rt = make_rt();
    let id = start_run(rt);
    let events = drain_events(rt, &id);
    let kinds: Vec<_> = events.iter().map(|e| event_kind(e)).collect();
    assert_eq!(kinds, vec!["started", "completed"],
        "journal must begin with started and end with completed");
    ancora_free_runtime(rt);
}

#[test]
fn journal_human_in_loop_event_order_matches_core_expectation() {
    let rt = make_rt();
    let id = start_run(rt);
    drain_events(rt, &id);
    let c_id = std::ffi::CString::new(id.as_str()).unwrap();
    ancora_run_resume(rt, c_id.as_ptr(), b"ok".as_ptr(), 2);
    let events = drain_events(rt, &id);
    let kinds: Vec<_> = events.iter().map(|e| event_kind(e)).collect();
    assert_eq!(kinds.first().map(|s| *s), Some("resumed"),
        "human-in-loop journal: first post-resume event must be resumed, got: {kinds:?}");
    ancora_free_runtime(rt);
}

#[test]
fn conformance_scenario_single_agent_id_matches_expected() {
    let scenario = &conformance::SINGLE_AGENT;
    assert_eq!(scenario.id, "single-agent", "single-agent scenario id must be 'single-agent'");
}

#[test]
fn conformance_scenario_human_in_loop_id_matches_expected() {
    let scenario = &conformance::HUMAN_IN_LOOP;
    assert_eq!(scenario.id, "human-in-loop");
}

#[test]
fn conformance_scenario_count_is_four() {
    assert_eq!(conformance::all_scenarios().len(), 4, "there must be exactly 4 conformance scenarios");
}

#[test]
fn ffi_run_start_null_ptr_guard() {
    let spec = b"{}";
    let mut out = AncorBuffer { ptr: std::ptr::null_mut(), len: 0 };
    let code = ancora_run_start(std::ptr::null_mut(), spec.as_ptr(), spec.len(), &mut out);
    assert_eq!(code, AncorErrorCode::NullPtr, "null rt must return NullPtr");
}

#[test]
fn ffi_run_cost_returns_ok_for_valid_run() {
    use ancora_ffi::run_ops::ancora_run_cost;
    let rt = make_rt();
    let id = start_run(rt);
    let c_id = std::ffi::CString::new(id.as_str()).unwrap();
    let mut cost = AncorBuffer { ptr: std::ptr::null_mut(), len: 0 };
    let code = ancora_run_cost(rt, c_id.as_ptr(), &mut cost);
    assert_eq!(code, AncorErrorCode::Ok);
    ancora_buffer_free(cost);
    ancora_free_runtime(rt);
}

#[test]
fn ffi_journal_equals_native_event_sequence_single_agent() {
    let rt = make_rt();
    let id = start_run(rt);
    let events = drain_events(rt, &id);
    let kinds: Vec<&str> = events.iter().map(|e| event_kind(e)).collect();
    let expected: &[&str] = &["started", "completed"];
    assert_eq!(kinds, expected,
        "FFI journal must equal native single-agent journal sequence");
    ancora_free_runtime(rt);
}

#[test]
fn ffi_journal_equals_native_event_sequence_after_resume() {
    let rt = make_rt();
    let id = start_run(rt);
    drain_events(rt, &id);
    let c_id = std::ffi::CString::new(id.as_str()).unwrap();
    ancora_run_resume(rt, c_id.as_ptr(), b"yes".as_ptr(), 3);
    let events = drain_events(rt, &id);
    let kinds: Vec<&str> = events.iter().map(|e| event_kind(e)).collect();
    assert!(kinds.iter().any(|k| *k == "resumed"),
        "FFI post-resume journal must contain resumed event, got: {kinds:?}");
    assert!(kinds.iter().any(|k| *k == "completed"),
        "FFI post-resume journal must end with completed event, got: {kinds:?}");
    ancora_free_runtime(rt);
}

#[test]
fn conformance_single_agent_tag_contains_agent() {
    let s = &conformance::SINGLE_AGENT;
    assert!(s.tags.contains(&"agent"), "single-agent scenario must have 'agent' tag");
}

#[test]
fn conformance_human_in_loop_tag_contains_suspend() {
    let s = &conformance::HUMAN_IN_LOOP;
    assert!(s.tags.contains(&"suspend"), "human-in-loop scenario must have 'suspend' tag");
}
