use crate::boot::{BootPhase, BootSequencer, next_phase, BOOT_SEQUENCE};

#[test]
fn test_service_starts_at_boot_all_pass() {
    let seq = BootSequencer::all_pass();
    let record = seq.run();
    assert!(record.all_succeeded(), "all boot phases should succeed");
}

#[test]
fn test_boot_reaches_ready_phase() {
    let seq = BootSequencer::all_pass();
    let record = seq.run();
    let phases: Vec<_> = record.phases.iter().map(|p| p.phase.clone()).collect();
    assert!(phases.contains(&BootPhase::Ready));
}

#[test]
fn test_boot_stops_on_failure() {
    let seq = BootSequencer::with_failure(BootPhase::CgroupSetup, "no cgroup v2");
    let record = seq.run();
    assert!(!record.all_succeeded());
    let failed = record.failed_phase().expect("should have a failed phase");
    assert_eq!(failed.phase, BootPhase::CgroupSetup);
}

#[test]
fn test_boot_sequence_order() {
    assert_eq!(BOOT_SEQUENCE[0], BootPhase::Init);
    assert_eq!(*BOOT_SEQUENCE.last().unwrap(), BootPhase::Ready);
}

#[test]
fn test_next_phase_transitions() {
    assert_eq!(next_phase(&BootPhase::Init), Some(BootPhase::ConfigLoad));
    assert_eq!(next_phase(&BootPhase::SocketBind), Some(BootPhase::Ready));
    assert_eq!(next_phase(&BootPhase::Ready), None);
}

#[test]
fn test_boot_phase_display() {
    assert_eq!(BootPhase::ModelPreload.to_string(), "model-preload");
    assert_eq!(BootPhase::Ready.to_string(), "ready");
}
