use crate::procedure::{OfflineProcedure, ProcedureStep};

#[test]
fn procedure_step_complete() {
    let mut step = ProcedureStep::new("s1", "Verify", "Check the data");
    step.complete(10);
    assert!(step.is_done());
    assert_eq!(step.completed_tick, Some(10));
}

#[test]
fn procedure_progress() {
    let mut p = OfflineProcedure::new("p1", "Import", "t1");
    p.add_step(ProcedureStep::new("s1", "S1", ""));
    p.add_step(ProcedureStep::new("s2", "S2", ""));
    p.get_step_mut("s1").unwrap().complete(1);
    assert_eq!(p.progress(), 0.5);
    assert!(!p.is_complete());
}

#[test]
fn procedure_is_complete() {
    let mut p = OfflineProcedure::new("p1", "Import", "t1");
    p.add_step(ProcedureStep::new("s1", "S1", ""));
    p.get_step_mut("s1").unwrap().complete(1);
    assert!(p.is_complete());
}

#[test]
fn procedure_counts() {
    let mut p = OfflineProcedure::new("p1", "Import", "t1");
    p.add_step(ProcedureStep::new("s1", "S1", ""));
    p.add_step(ProcedureStep::new("s2", "S2", ""));
    assert_eq!(p.step_count(), 2);
    assert_eq!(p.pending_count(), 2);
    assert_eq!(p.completed_count(), 0);
}
