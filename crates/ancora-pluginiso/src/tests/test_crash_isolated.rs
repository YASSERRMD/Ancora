use crate::crash_isolation::{CrashIsolationHandle, CrashIsolationMode, PluginHealth};

#[test]
fn plugin_crash_does_not_crash_host_in_isolated_mode() {
    let mut handle = CrashIsolationHandle::new("plugin-x", CrashIsolationMode::Isolated);

    // Simulate a plugin crash.
    let should_crash_host = handle.record_crash("null pointer dereference");

    // Host must NOT be told to crash.
    assert!(!should_crash_host, "isolated crash must not propagate to host");
    // Plugin should be marked as faulted.
    assert!(!handle.is_healthy());
    assert!(matches!(handle.health, PluginHealth::Faulted { .. }));
}

#[test]
fn crashed_plugin_can_be_restarted() {
    let mut handle = CrashIsolationHandle::new("plugin-y", CrashIsolationMode::Isolated);
    handle.record_crash("oom");
    assert!(!handle.is_healthy());

    // Restart the plugin.
    handle.reset();
    assert!(handle.is_healthy());
}

#[test]
fn multiple_crashes_keep_plugin_faulted_until_reset() {
    let mut handle = CrashIsolationHandle::new("plugin-z", CrashIsolationMode::Isolated);
    handle.record_crash("crash-1");
    handle.record_crash("crash-2");
    assert!(!handle.is_healthy());

    handle.reset();
    assert!(handle.is_healthy());
}

#[test]
fn propagate_mode_signals_host_to_crash() {
    let mut handle = CrashIsolationHandle::new("trusted-builtin", CrashIsolationMode::Propagate);
    let should_crash_host = handle.record_crash("fatal error");
    assert!(should_crash_host, "propagate mode must signal the host");
}

#[test]
fn stopped_plugin_is_not_healthy() {
    let mut handle = CrashIsolationHandle::new("plugin-w", CrashIsolationMode::Isolated);
    handle.stop();
    assert!(!handle.is_healthy());
    assert_eq!(handle.health, PluginHealth::Stopped);
}
