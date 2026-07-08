use crate::audit::{ThreatIntelAction, ThreatIntelAuditEntry, ThreatIntelAuditLog};

#[test]
fn audit_record_and_count() {
    let mut log = ThreatIntelAuditLog::new();
    log.record(ThreatIntelAuditEntry::new(
        1,
        "t1",
        ThreatIntelAction::IndicatorAdded,
        "i1",
        "",
    ));
    log.record(ThreatIntelAuditEntry::new(
        2,
        "t1",
        ThreatIntelAction::FeedIngested,
        "f1",
        "",
    ));
    assert_eq!(log.count(), 2);
}

#[test]
fn audit_for_tenant() {
    let mut log = ThreatIntelAuditLog::new();
    log.record(ThreatIntelAuditEntry::new(
        1,
        "t1",
        ThreatIntelAction::IndicatorAdded,
        "i1",
        "",
    ));
    log.record(ThreatIntelAuditEntry::new(
        2,
        "t2",
        ThreatIntelAction::IndicatorAdded,
        "i2",
        "",
    ));
    assert_eq!(log.for_tenant("t1").len(), 1);
}

#[test]
fn audit_by_action() {
    let mut log = ThreatIntelAuditLog::new();
    log.record(ThreatIntelAuditEntry::new(
        1,
        "t1",
        ThreatIntelAction::IndicatorAdded,
        "i1",
        "",
    ));
    log.record(ThreatIntelAuditEntry::new(
        2,
        "t1",
        ThreatIntelAction::FeedIngested,
        "f1",
        "",
    ));
    log.record(ThreatIntelAuditEntry::new(
        3,
        "t1",
        ThreatIntelAction::IndicatorAdded,
        "i3",
        "",
    ));
    assert_eq!(log.by_action(&ThreatIntelAction::IndicatorAdded).len(), 2);
}

#[test]
fn audit_all_iterator() {
    let mut log = ThreatIntelAuditLog::new();
    for k in 0..4u64 {
        log.record(ThreatIntelAuditEntry::new(
            k,
            "t1",
            ThreatIntelAction::ScoreComputed,
            "x",
            "",
        ));
    }
    assert_eq!(log.all().count(), 4);
}
