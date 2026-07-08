use crate::wakeup::{EventWakeup, ScheduledWakeup};

#[test]
fn scheduled_wakeup_fires_at_tick() {
    let w = ScheduledWakeup::new("r1", 100);
    assert!(!w.should_fire(99));
    assert!(w.should_fire(100));
    assert!(w.should_fire(200));
}

#[test]
fn event_wakeup_resumes_run() {
    let mut w = EventWakeup::new("r1", "data-ready");
    assert!(!w.has_fired());
    assert!(w.trigger("data-ready"));
    assert!(w.has_fired());
}

#[test]
fn event_wakeup_wrong_event_does_not_fire() {
    let mut w = EventWakeup::new("r1", "data-ready");
    assert!(!w.trigger("other-event"));
    assert!(!w.has_fired());
}

#[test]
fn event_wakeup_fires_only_once() {
    let mut w = EventWakeup::new("r1", "go");
    w.trigger("go");
    assert!(!w.trigger("go"));
}
