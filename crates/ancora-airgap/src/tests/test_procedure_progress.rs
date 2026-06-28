use crate::procedure::{OfflineProcedure, ProcedureStep};

#[test]
fn progress_zero_steps() {
    let p = OfflineProcedure::new("p1", "Empty", "t1");
    assert_eq!(p.progress(), 0.0);
    assert!(!p.is_complete());
}

#[test]
fn progress_all_complete() {
    let mut p = OfflineProcedure::new("p1", "Proc", "t1");
    p.add_step(ProcedureStep::new("s1", "S1", ""));
    p.add_step(ProcedureStep::new("s2", "S2", ""));
    p.get_step_mut("s1").unwrap().complete(1);
    p.get_step_mut("s2").unwrap().complete(2);
    assert_eq!(p.progress(), 1.0);
    assert!(p.is_complete());
}

#[test]
fn step_skip_counts_as_done() {
    let mut p = OfflineProcedure::new("p1", "Proc", "t1");
    p.add_step(ProcedureStep::new("s1", "S1", ""));
    p.get_step_mut("s1").unwrap().skip();
    assert!(p.is_complete());
}

#[test]
fn step_fail_counts_as_done() {
    let mut step = ProcedureStep::new("s1", "S1", "");
    step.fail();
    assert!(step.is_done());
}
