use crate::builder::{IncidentBuilder, RunbookBuilder};
use crate::incident::Severity;

#[test]
fn incident_builder_defaults() {
    let i = IncidentBuilder::new("i1", "t1", "Title").build();
    assert_eq!(i.id, "i1");
    assert_eq!(i.tenant_id, "t1");
    assert_eq!(i.severity, Severity::Medium);
    assert_eq!(i.detected_tick, 0);
}

#[test]
fn incident_builder_custom() {
    let i = IncidentBuilder::new("i2", "t2", "Test")
        .severity(Severity::Critical)
        .tick(500)
        .build();
    assert_eq!(i.severity, Severity::Critical);
    assert_eq!(i.detected_tick, 500);
}

#[test]
fn runbook_builder() {
    let rb = RunbookBuilder::new("rb1", "Runbook", "i1")
        .step("s1", "Step 1", "Desc 1")
        .step("s2", "Step 2", "Desc 2")
        .build();
    assert_eq!(rb.step_count(), 2);
    assert_eq!(rb.incident_id, "i1");
    assert_eq!(rb.id, "rb1");
}

#[test]
fn runbook_builder_empty() {
    let rb = RunbookBuilder::new("rb1", "Empty", "i1").build();
    assert_eq!(rb.step_count(), 0);
    assert!(!rb.is_complete());
}
