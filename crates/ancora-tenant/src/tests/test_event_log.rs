use crate::{TenantEvent, TenantEventKind, TenantEventLog};
#[test]
fn event_log_records_events() {
    let mut log = TenantEventLog::new();
    log.record(TenantEvent::new(1, "t1", TenantEventKind::Registered));
    log.record(TenantEvent::new(2, "t1", TenantEventKind::Activated));
    assert_eq!(log.count(), 2);
}
#[test]
fn events_for_filters_by_tenant() {
    let mut log = TenantEventLog::new();
    log.record(TenantEvent::new(1, "t1", TenantEventKind::Registered));
    log.record(TenantEvent::new(2, "t2", TenantEventKind::Registered));
    log.record(TenantEvent::new(3, "t1", TenantEventKind::Activated));
    let t1_events = log.events_for("t1");
    assert_eq!(t1_events.len(), 2);
}
#[test]
fn events_of_kind_filters_by_kind() {
    let mut log = TenantEventLog::new();
    log.record(TenantEvent::new(1, "t1", TenantEventKind::Registered));
    log.record(TenantEvent::new(2, "t1", TenantEventKind::Suspended));
    log.record(TenantEvent::new(3, "t2", TenantEventKind::Suspended));
    let suspended = log.events_of_kind(&TenantEventKind::Suspended);
    assert_eq!(suspended.len(), 2);
}
