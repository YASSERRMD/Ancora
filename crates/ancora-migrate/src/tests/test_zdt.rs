use crate::zero_downtime::{ZdtMigration, ZdtPhase};

#[test]
fn starts_idle() {
    let m = ZdtMigration::new("add-column", 100);
    assert_eq!(m.phase, ZdtPhase::Idle);
}

#[test]
fn expand_then_backfill_sequence() {
    let mut m = ZdtMigration::new("m", 50);
    m.start_expand();
    assert_eq!(m.phase, ZdtPhase::Expand);
    m.start_backfill();
    assert_eq!(m.phase, ZdtPhase::Backfill);
}

#[test]
fn backfill_progress_and_complete() {
    let mut m = ZdtMigration::new("m", 100);
    m.start_backfill();
    m.advance_backfill(60);
    assert!(!m.backfill_complete());
    m.advance_backfill(40);
    assert!(m.backfill_complete());
    assert!((m.progress_pct() - 100.0).abs() < 0.001);
}

#[test]
fn contract_only_after_backfill_complete() {
    let mut m = ZdtMigration::new("m", 10);
    m.start_backfill();
    m.start_contract(); // no-op: not done
    assert_eq!(m.phase, ZdtPhase::Backfill);
    m.advance_backfill(10);
    m.start_contract();
    assert_eq!(m.phase, ZdtPhase::Contract);
}

#[test]
fn finish_sets_done() {
    let mut m = ZdtMigration::new("m", 0);
    m.start_contract();
    m.finish();
    assert_eq!(m.phase, ZdtPhase::Done);
}
