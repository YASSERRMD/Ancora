/// Tests for the audit log that tracks plugin lifecycle events.
///
/// An audit log entry records who did what to which plugin and when.  The host
/// appends an entry for every lifecycle event: load, unload, crash, policy
/// violation, signature check result, etc.

use crate::audit::{AuditEvent, AuditLog, EventKind};

#[test]
fn new_audit_log_is_empty() {
    let log = AuditLog::new();
    assert_eq!(log.len(), 0);
}

#[test]
fn plugin_load_recorded() {
    let mut log = AuditLog::new();
    log.record(AuditEvent {
        plugin_id: "plugin-a".into(),
        kind: EventKind::Loaded,
        detail: "wasm runtime, version 1.0.0".into(),
    });
    assert_eq!(log.len(), 1);
    let entry = &log.entries()[0];
    assert_eq!(entry.plugin_id, "plugin-a");
    assert!(matches!(entry.kind, EventKind::Loaded));
}

#[test]
fn policy_violation_recorded() {
    let mut log = AuditLog::new();
    log.record(AuditEvent {
        plugin_id: "plugin-b".into(),
        kind: EventKind::PolicyViolation,
        detail: "attempted network access to blocked.example.com:443".into(),
    });
    assert_eq!(log.len(), 1);
    assert!(matches!(log.entries()[0].kind, EventKind::PolicyViolation));
}

#[test]
fn multiple_events_accumulated() {
    let mut log = AuditLog::new();
    log.record(AuditEvent { plugin_id: "p".into(), kind: EventKind::Loaded, detail: "".into() });
    log.record(AuditEvent { plugin_id: "p".into(), kind: EventKind::Crashed, detail: "oom".into() });
    log.record(AuditEvent { plugin_id: "p".into(), kind: EventKind::Unloaded, detail: "".into() });
    assert_eq!(log.len(), 3);
}

#[test]
fn filter_by_plugin_id() {
    let mut log = AuditLog::new();
    log.record(AuditEvent { plugin_id: "alpha".into(), kind: EventKind::Loaded, detail: "".into() });
    log.record(AuditEvent { plugin_id: "beta".into(), kind: EventKind::Loaded, detail: "".into() });
    log.record(AuditEvent { plugin_id: "alpha".into(), kind: EventKind::Unloaded, detail: "".into() });

    let alpha_events = log.events_for_plugin("alpha");
    assert_eq!(alpha_events.len(), 2);
    for e in alpha_events {
        assert_eq!(e.plugin_id, "alpha");
    }
}

#[test]
fn crash_event_detail_preserved() {
    let mut log = AuditLog::new();
    log.record(AuditEvent {
        plugin_id: "crasher".into(),
        kind: EventKind::Crashed,
        detail: "stack overflow in render_frame".into(),
    });
    let entry = &log.entries()[0];
    assert!(entry.detail.contains("stack overflow"));
}
