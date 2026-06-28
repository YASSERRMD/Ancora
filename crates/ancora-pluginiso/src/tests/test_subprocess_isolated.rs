use crate::capability_grants::CapabilityGrants;
use crate::crash_isolation::CrashIsolationMode;
use crate::filesystem_policy::FilesystemPolicy;
use crate::network_policy::NetworkPolicy;
use crate::resource_limits::ResourceLimits;
use crate::sandbox::{RuntimeKind, Sandbox};
use crate::signature::SignaturePolicy;
use crate::subprocess_runtime::{PluginRequest, ResponseStatus, SubprocessInstance};

fn make_subprocess_sandbox() -> Sandbox {
    Sandbox::new(
        "subprocess-isolation-test",
        RuntimeKind::Subprocess,
        ResourceLimits::default(),
        NetworkPolicy::deny_all(),
        FilesystemPolicy::deny_all(),
        CapabilityGrants::none(),
        CrashIsolationMode::Isolated,
        SignaturePolicy::Required,
    )
}

#[test]
fn subprocess_plugin_isolated_from_host() {
    let sb = make_subprocess_sandbox();
    // Verify isolation mode is set.
    assert_eq!(sb.crash_isolation, CrashIsolationMode::Isolated);
}

#[test]
fn subprocess_plugin_responds_to_requests() {
    let mut inst = SubprocessInstance::spawn(
        "inst-1",
        &"/usr/local/bin/my-plugin".to_string(),
        make_subprocess_sandbox(),
    )
    .expect("spawn ok");

    let resp = inst
        .send(PluginRequest { method: "compute".into(), payload: b"data".to_vec() })
        .expect("send ok");

    assert_eq!(resp.status, ResponseStatus::Ok);
    assert!(inst.is_running());
}

#[test]
fn subprocess_stops_cleanly() {
    let mut inst = SubprocessInstance::spawn(
        "inst-2",
        &"/usr/local/bin/my-plugin".to_string(),
        make_subprocess_sandbox(),
    )
    .expect("spawn ok");

    assert!(inst.is_running());
    inst.terminate();
    assert!(!inst.is_running());
}

#[test]
fn subprocess_stops_accepting_requests_after_termination() {
    let mut inst = SubprocessInstance::spawn(
        "inst-3",
        &"/usr/local/bin/my-plugin".to_string(),
        make_subprocess_sandbox(),
    )
    .expect("spawn ok");

    inst.terminate();
    let result = inst.send(PluginRequest { method: "op".into(), payload: vec![] });
    assert!(result.is_err(), "terminated subprocess must reject requests");
}

#[test]
fn subprocess_empty_executable_rejected() {
    let result = SubprocessInstance::spawn("id", &"".to_string(), make_subprocess_sandbox());
    assert!(result.is_err());
}
