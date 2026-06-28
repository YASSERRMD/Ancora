use crate::presets::data_import_procedure;

#[test]
fn data_import_has_five_steps() {
    let p = data_import_procedure("t1");
    assert_eq!(p.step_count(), 5);
}

#[test]
fn data_import_all_pending_initially() {
    let p = data_import_procedure("t1");
    assert_eq!(p.pending_count(), 5);
    assert_eq!(p.completed_count(), 0);
}

#[test]
fn data_import_not_complete_initially() {
    let p = data_import_procedure("t1");
    assert!(!p.is_complete());
}
