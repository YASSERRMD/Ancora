use crate::readiness::{build_default_checklist, CheckStatus, ChecklistItem, ReadinessChecklist};

#[test]
fn test_default_checklist_has_items() {
    let checklist = build_default_checklist();
    assert!(!checklist.items.is_empty());
}

#[test]
fn test_default_checklist_all_fail() {
    let checklist = build_default_checklist();
    assert_eq!(checklist.pass_count(), 0);
    assert_eq!(checklist.fail_count(), checklist.items.len());
    assert!(!checklist.is_ready());
}

#[test]
fn test_checklist_pass_all() {
    let mut checklist = ReadinessChecklist::default();
    checklist.add(ChecklistItem::new("obs-001", "Tracing enabled").pass());
    checklist.add(ChecklistItem::new("eval-001", "Eval platform connected").pass());
    assert_eq!(checklist.pass_count(), 2);
    assert_eq!(checklist.fail_count(), 0);
    assert!(checklist.is_ready());
}

#[test]
fn test_checklist_na_not_counted_as_fail() {
    let mut checklist = ReadinessChecklist::default();
    checklist.add(ChecklistItem::new("obs-001", "Tracing enabled").pass());
    checklist.add(ChecklistItem::new("opt-001", "Optional feature").na());
    assert_eq!(checklist.fail_count(), 0);
    assert!(checklist.is_ready());
}

#[test]
fn test_checklist_summary_format() {
    let mut checklist = ReadinessChecklist::default();
    checklist.add(ChecklistItem::new("obs-001", "Tracing enabled").pass());
    checklist.add(ChecklistItem::new("obs-002", "Metrics enabled").pass());
    checklist.add(ChecklistItem::new("obs-003", "Logging connected").pass());
    let summary = checklist.summary();
    assert!(summary.contains("3/3"));
}

#[test]
fn test_checklist_item_with_notes() {
    let item = ChecklistItem::new("obs-001", "Tracing enabled")
        .pass()
        .with_notes("Verified via OTLP exporter logs");
    assert_eq!(item.status, CheckStatus::Pass);
    assert!(item.notes.is_some());
}

#[test]
fn test_checklist_item_fail_by_default() {
    let item = ChecklistItem::new("obs-001", "Tracing enabled");
    assert_eq!(item.status, CheckStatus::Fail);
}

#[test]
fn test_default_checklist_expected_count() {
    let checklist = build_default_checklist();
    // The default checklist should have exactly 15 items.
    assert_eq!(checklist.items.len(), 15);
}

#[test]
fn test_checklist_obs_items_present() {
    let checklist = build_default_checklist();
    let obs_items: Vec<&str> = checklist
        .items
        .iter()
        .filter(|i| i.id.starts_with("obs-"))
        .map(|i| i.id.as_str())
        .collect();
    assert!(!obs_items.is_empty());
    assert!(obs_items.contains(&"obs-001"));
}

#[test]
fn test_checklist_eval_items_present() {
    let checklist = build_default_checklist();
    let eval_items: Vec<&str> = checklist
        .items
        .iter()
        .filter(|i| i.id.starts_with("eval-"))
        .map(|i| i.id.as_str())
        .collect();
    assert!(!eval_items.is_empty());
    assert!(eval_items.contains(&"eval-001"));
}

#[test]
fn test_check_status_display() {
    assert_eq!(CheckStatus::Pass.to_string(), "PASS");
    assert_eq!(CheckStatus::Fail.to_string(), "FAIL");
    assert_eq!(CheckStatus::NotApplicable.to_string(), "N/A");
}
