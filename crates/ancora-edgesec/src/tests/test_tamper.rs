use crate::tamper::{TamperEvent, TamperEventKind, TamperMonitor};

#[test]
fn test_tamper_detected() {
    let mut monitor = TamperMonitor::new(100);
    let event = TamperEvent::new("device-001", TamperEventKind::HashMismatch, "boot hash mismatch", 1);
    monitor.record(event);
    assert!(monitor.is_tampered("device-001"));
}

#[test]
fn test_tamper_not_detected_for_clean_device() {
    let monitor = TamperMonitor::new(100);
    assert!(!monitor.is_tampered("clean-device"));
}

#[test]
fn test_tamper_check_hash_mismatch() {
    let mut monitor = TamperMonitor::new(100);
    let expected = vec![0xAA; 32];
    let measured = vec![0xBB; 32];
    let ok = monitor.check_hash("device-002", "firmware", &expected, &measured, 5);
    assert!(!ok);
    assert!(monitor.is_tampered("device-002"));
}

#[test]
fn test_tamper_check_hash_match() {
    let mut monitor = TamperMonitor::new(100);
    let hash = vec![0xCC; 32];
    let ok = monitor.check_hash("device-003", "bootloader", &hash, &hash, 6);
    assert!(ok);
    assert!(!monitor.is_tampered("device-003"));
}

#[test]
fn test_tamper_events_for_device() {
    let mut monitor = TamperMonitor::new(100);
    monitor.record(TamperEvent::new("dev-A", TamperEventKind::UnexpectedReboot, "reboot", 1));
    monitor.record(TamperEvent::new("dev-B", TamperEventKind::ClockSkew, "clock", 2));
    let events = monitor.events_for("dev-A");
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].kind, TamperEventKind::UnexpectedReboot);
}
