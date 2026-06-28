use crate::{AttestationLog, AttestationRecord, AttestationStatus, BootChain, BootPolicy, IntegrityReport, Measurement, MeasurementKind};
#[test]
fn report_pass_with_trusted_attestations() {
    let policy = BootPolicy::new("t1").allow_digest("vmlinuz", "good").allow_unknown();
    let mut chain = BootChain::new("t1", "n1");
    chain.add_step(Measurement::new("m1", MeasurementKind::Kernel, "vmlinuz", "good", 0));
    let mut attest = AttestationLog::new();
    attest.record(AttestationRecord::new("a1", "t1", "n1", AttestationStatus::Trusted, "q", 0));
    let report = IntegrityReport::generate(&policy, &chain, &attest, 10);
    assert!(report.is_fully_trusted());
    assert_eq!(report.trusted_attestations, 1);
}
