use crate::detection::{DetectionEvent, DetectionLog, DetectionSource};

fn ev(id: &str, scenario_id: &str, tp: bool) -> DetectionEvent {
    DetectionEvent::new(id, scenario_id, DetectionSource::Siem, "desc", 1, tp)
}

#[test]
fn empty() {
    let log = DetectionLog::new();
    assert_eq!(log.count(), 0);
}

#[test]
fn record_and_count() {
    let mut log = DetectionLog::new();
    log.record(ev("d1", "sc1", true));
    log.record(ev("d2", "sc1", false));
    assert_eq!(log.count(), 2);
}

#[test]
fn for_scenario() {
    let mut log = DetectionLog::new();
    log.record(ev("d1", "sc1", true));
    log.record(ev("d2", "sc2", true));
    assert_eq!(log.for_scenario("sc1").len(), 1);
}

#[test]
fn by_source() {
    let mut log = DetectionLog::new();
    log.record(DetectionEvent::new("d1", "sc1", DetectionSource::Edr, "x", 1, true));
    log.record(DetectionEvent::new("d2", "sc1", DetectionSource::Siem, "x", 1, false));
    assert_eq!(log.by_source(&DetectionSource::Edr).len(), 1);
    assert_eq!(log.by_source(&DetectionSource::Ids).len(), 0);
}
