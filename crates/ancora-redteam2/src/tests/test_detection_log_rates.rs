use crate::detection::{DetectionEvent, DetectionLog, DetectionSource};

#[test]
fn empty_rate_is_zero() {
    let log = DetectionLog::new();
    assert!((log.detection_rate() - 0.0).abs() < f64::EPSILON);
}

#[test]
fn all_true_positive() {
    let mut log = DetectionLog::new();
    log.record(DetectionEvent::new("d1", "sc1", DetectionSource::Edr, "x", 1, true));
    log.record(DetectionEvent::new("d2", "sc1", DetectionSource::Edr, "x", 2, true));
    assert!((log.detection_rate() - 1.0).abs() < 1e-9);
    assert_eq!(log.false_positives().len(), 0);
}

#[test]
fn mixed_rate() {
    let mut log = DetectionLog::new();
    log.record(DetectionEvent::new("d1", "sc1", DetectionSource::Siem, "x", 1, true));
    log.record(DetectionEvent::new("d2", "sc1", DetectionSource::Siem, "x", 2, false));
    assert!((log.detection_rate() - 0.5).abs() < 1e-9);
    assert_eq!(log.true_positives().len(), 1);
    assert_eq!(log.false_positives().len(), 1);
}
