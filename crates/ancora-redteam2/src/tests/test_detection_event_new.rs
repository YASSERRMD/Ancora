use crate::detection::{DetectionEvent, DetectionSource};

#[test]
fn basic_fields() {
    let ev = DetectionEvent::new("d1", "sc1", DetectionSource::Edr, "Suspicious process", 5, true);
    assert_eq!(ev.id, "d1");
    assert_eq!(ev.scenario_id, "sc1");
    assert_eq!(ev.source, DetectionSource::Edr);
    assert_eq!(ev.description, "Suspicious process");
    assert_eq!(ev.tick, 5);
    assert!(ev.true_positive);
    assert!(ev.step_id.is_none());
}

#[test]
fn with_step() {
    let ev = DetectionEvent::new("d1", "sc1", DetectionSource::Ids, "Port scan detected", 2, false)
        .with_step("step-42");
    assert_eq!(ev.step_id.as_deref(), Some("step-42"));
}
