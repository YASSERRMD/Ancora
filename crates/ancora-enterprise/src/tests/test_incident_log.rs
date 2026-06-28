use crate::incident::{EnterpriseIncident, IncidentLog, IncidentSeverity};

fn inc(id: &str, tenant_id: &str, sev: IncidentSeverity) -> EnterpriseIncident {
    EnterpriseIncident::new(id, tenant_id, "N", sev, "domain", 1)
}

#[test]
fn empty_log() {
    let log = IncidentLog::new();
    assert_eq!(log.count(), 0);
    assert_eq!(log.open().len(), 0);
}

#[test]
fn record_and_open() {
    let mut log = IncidentLog::new();
    log.record(inc("i1", "t1", IncidentSeverity::High));
    log.record(inc("i2", "t1", IncidentSeverity::Low));
    assert_eq!(log.count(), 2);
    assert_eq!(log.open().len(), 2);
}

#[test]
fn critical_filter() {
    let mut log = IncidentLog::new();
    log.record(inc("i1", "t1", IncidentSeverity::Critical));
    log.record(inc("i2", "t1", IncidentSeverity::Low));
    assert_eq!(log.critical().len(), 1);
}

#[test]
fn for_tenant_filter() {
    let mut log = IncidentLog::new();
    log.record(inc("i1", "t1", IncidentSeverity::High));
    log.record(inc("i2", "t2", IncidentSeverity::High));
    assert_eq!(log.for_tenant("t1").len(), 1);
}

#[test]
fn resolved_filter() {
    let mut log = IncidentLog::new();
    log.record(inc("i1", "t1", IncidentSeverity::High));
    log.get_mut("i1").unwrap().resolve(100);
    log.record(inc("i2", "t1", IncidentSeverity::Low));
    assert_eq!(log.resolved().len(), 1);
    assert_eq!(log.open().len(), 1);
}
