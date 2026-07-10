//! End-to-end proof that a real, provider-backed run can suspend at an
//! approval-gated tool call and resume to completion through the exact FFI
//! surface a host language SDK uses: `ancora_runtime_new_with_config`,
//! `ancora_tool_register_requires_approval`, `ancora_run_start`,
//! `ancora_run_poll`, and `ancora_run_resume`.

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::os::raw::c_char;

use ancora_ffi::buffer::{ancora_buffer_free, AncorBuffer};
use ancora_ffi::error_code::AncorErrorCode;
use ancora_ffi::run_ops::{ancora_run_poll, ancora_run_resume, ancora_run_start};
use ancora_ffi::runtime::{ancora_free_runtime, ancora_runtime_new_with_config};
use ancora_ffi::tool_ops::ancora_tool_register_requires_approval;

fn cstr(s: &str) -> std::ffi::CString {
    std::ffi::CString::new(s).unwrap()
}

fn buf_to_string(buf: &AncorBuffer) -> String {
    if buf.ptr.is_null() || buf.len == 0 {
        return String::new();
    }
    let slice = unsafe { std::slice::from_raw_parts(buf.ptr, buf.len) };
    String::from_utf8_lossy(slice).into_owned()
}

fn read_full_http_request(stream: &mut TcpStream) -> String {
    let mut buf = Vec::new();
    let mut chunk = [0u8; 4096];
    let headers_end = loop {
        let n = stream.read(&mut chunk).unwrap_or(0);
        if n == 0 {
            break None;
        }
        buf.extend_from_slice(&chunk[..n]);
        if let Some(pos) = buf.windows(4).position(|w| w == b"\r\n\r\n") {
            break Some(pos + 4);
        }
    };
    let Some(headers_end) = headers_end else {
        return String::from_utf8_lossy(&buf).into_owned();
    };
    let headers = String::from_utf8_lossy(&buf[..headers_end]);
    let content_length: usize = headers
        .lines()
        .find_map(|l| {
            l.to_ascii_lowercase()
                .strip_prefix("content-length:")
                .map(|v| v.trim().to_owned())
        })
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);
    while buf.len() < headers_end + content_length {
        let n = stream.read(&mut chunk).unwrap_or(0);
        if n == 0 {
            break;
        }
        buf.extend_from_slice(&chunk[..n]);
    }
    String::from_utf8_lossy(&buf).into_owned()
}

/// Serves each body in `responses` to one sequential connection, in order --
/// the first response for the initial run_start's model call, the second
/// for the model call resume triggers, etc.
fn mock_server(responses: Vec<&'static str>) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();
    std::thread::spawn(move || {
        for body in responses {
            if let Ok((mut stream, _)) = listener.accept() {
                read_full_http_request(&mut stream);
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                    body.len(),
                    body
                );
                let _ = stream.write_all(response.as_bytes());
            }
        }
    });
    format!("http://{addr}")
}

const TOOL_CALL_RESPONSE: &str = r#"{"choices":[{"message":{"role":"assistant","content":null,"tool_calls":[{"id":"call_1","type":"function","function":{"name":"get_weather","arguments":"{\"city\":\"Paris\"}"}}]},"finish_reason":"tool_calls"}],"usage":{"prompt_tokens":20,"completion_tokens":8}}"#;
const FINAL_TEXT_RESPONSE: &str = r#"{"choices":[{"message":{"role":"assistant","content":"it is sunny in Paris","tool_calls":[]},"finish_reason":"stop"}],"usage":{"prompt_tokens":30,"completion_tokens":6}}"#;

unsafe extern "C" fn unreachable_callback(
    _input: *const u8,
    _input_len: usize,
    _out: *mut AncorBuffer,
) -> AncorErrorCode {
    panic!("gated tool callback must not be invoked before a decision is supplied");
}

#[test]
fn real_run_suspends_at_gated_tool_call_and_resumes_to_completion() {
    let base_url = mock_server(vec![TOOL_CALL_RESPONSE, FINAL_TEXT_RESPONSE]);
    let config = format!(r#"{{"provider":{{"base_url":"{base_url}"}}}}"#);
    let mut rt = std::ptr::null_mut();
    unsafe {
        ancora_runtime_new_with_config(config.as_ptr(), config.len(), &mut rt);
    }

    let tool_name = cstr("get_weather");
    let code = unsafe {
        ancora_tool_register_requires_approval(rt, tool_name.as_ptr(), unreachable_callback)
    };
    assert_eq!(code, AncorErrorCode::Ok);

    let spec = br#"{"model_id":"mock","instructions":"what is the weather"}"#;
    let mut out = AncorBuffer {
        ptr: std::ptr::null_mut(),
        len: 0,
    };
    unsafe { ancora_run_start(rt, spec.as_ptr(), spec.len(), &mut out) };
    let run_id = buf_to_string(&out);
    unsafe { ancora_buffer_free(out) };
    let c_run_id = cstr(&run_id);

    let poll = |rt: *mut ancora_ffi::handles::AncorRuntime, c_run_id: *const c_char| {
        let mut event = AncorBuffer {
            ptr: std::ptr::null_mut(),
            len: 0,
        };
        unsafe { ancora_run_poll(rt, c_run_id, &mut event) };
        let s = buf_to_string(&event);
        unsafe { ancora_buffer_free(event) };
        s
    };

    let started = poll(rt, c_run_id.as_ptr());
    assert!(started.contains("\"started\""), "got: {started}");

    let suspended = poll(rt, c_run_id.as_ptr());
    assert!(
        suspended.contains("\"suspended\""),
        "expected a suspended event, got: {suspended}"
    );
    assert!(suspended.contains("get_weather"), "got: {suspended}");
    assert!(suspended.contains("call_1"), "got: {suspended}");

    // Nothing more queued while suspended.
    assert_eq!(poll(rt, c_run_id.as_ptr()), "");

    let decision = br#"{"result_json":"\"72F and sunny\"","is_error":false}"#;
    unsafe { ancora_run_resume(rt, c_run_id.as_ptr(), decision.as_ptr(), decision.len()) };

    let resumed = poll(rt, c_run_id.as_ptr());
    assert!(resumed.contains("\"resumed\""), "got: {resumed}");

    let completed = poll(rt, c_run_id.as_ptr());
    assert!(
        completed.contains("it is sunny in Paris"),
        "expected the real model response after resume, got: {completed}"
    );

    unsafe { ancora_free_runtime(rt) };
}

#[test]
fn resume_on_never_suspended_run_keeps_legacy_synthetic_behavior() {
    // A default (offline) run never suspends -- resuming it must keep the
    // pre-existing harmless resumed/completed synthetic pair so callers
    // that resume defensively/idempotently keep working unchanged.
    use ancora_ffi::runtime::{ancora_free_runtime as free_rt, ancora_runtime_new};

    let mut rt = std::ptr::null_mut();
    unsafe { ancora_runtime_new(&mut rt) };

    let spec = br#"{"model_id":"mock","instructions":"hello"}"#;
    let mut out = AncorBuffer {
        ptr: std::ptr::null_mut(),
        len: 0,
    };
    unsafe { ancora_run_start(rt, spec.as_ptr(), spec.len(), &mut out) };
    let run_id = buf_to_string(&out);
    unsafe { ancora_buffer_free(out) };
    let c_run_id = cstr(&run_id);

    // Drain started + completed.
    for _ in 0..2 {
        let mut event = AncorBuffer {
            ptr: std::ptr::null_mut(),
            len: 0,
        };
        unsafe { ancora_run_poll(rt, c_run_id.as_ptr(), &mut event) };
        unsafe { ancora_buffer_free(event) };
    }

    let decision = b"approve";
    unsafe { ancora_run_resume(rt, c_run_id.as_ptr(), decision.as_ptr(), decision.len()) };

    let mut event = AncorBuffer {
        ptr: std::ptr::null_mut(),
        len: 0,
    };
    unsafe { ancora_run_poll(rt, c_run_id.as_ptr(), &mut event) };
    let resumed = buf_to_string(&event);
    unsafe { ancora_buffer_free(event) };
    assert!(resumed.contains("\"resumed\""), "got: {resumed}");

    unsafe { free_rt(rt) };
}
