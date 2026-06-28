use crate::timeline::{IncidentTimeline, TimelineEvent, TimelineEventKind};

#[test]
fn timeline_add_and_query() {
    let mut tl = IncidentTimeline::new();
    tl.add(TimelineEvent::new("i1", TimelineEventKind::Detected, "system", "Detected", 1));
    tl.add(TimelineEvent::new("i1", TimelineEventKind::Assigned, "alice", "Assigned to alice", 2));
    tl.add(TimelineEvent::new("i2", TimelineEventKind::Detected, "system", "Other", 3));
    assert_eq!(tl.count(), 3);
    assert_eq!(tl.for_incident("i1").len(), 2);
    assert_eq!(tl.for_incident("i2").len(), 1);
}

#[test]
fn timeline_by_kind() {
    let mut tl = IncidentTimeline::new();
    tl.add(TimelineEvent::new("i1", TimelineEventKind::Note, "alice", "Note 1", 1));
    tl.add(TimelineEvent::new("i1", TimelineEventKind::Note, "bob", "Note 2", 2));
    tl.add(TimelineEvent::new("i1", TimelineEventKind::Resolved, "alice", "Done", 3));
    assert_eq!(tl.by_kind(&TimelineEventKind::Note).len(), 2);
    assert_eq!(tl.by_kind(&TimelineEventKind::Resolved).len(), 1);
}

#[test]
fn timeline_all() {
    let mut tl = IncidentTimeline::new();
    for i in 0..5 {
        tl.add(TimelineEvent::new("i1", TimelineEventKind::Note, "u", format!("n{}", i), i));
    }
    assert_eq!(tl.all().len(), 5);
}
