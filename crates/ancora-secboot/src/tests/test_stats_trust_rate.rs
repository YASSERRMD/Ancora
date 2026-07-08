use crate::{
    AttestationLog, AttestationRecord, AttestationStatus, BootChain, BootStats, Measurement,
    MeasurementKind,
};
#[test]
fn trust_rate_calculated_correctly() {
    let mut chain = BootChain::new("t1", "n1");
    chain.add_step(Measurement::new("m1", MeasurementKind::Kernel, "k", "d", 0));
    let mut attest = AttestationLog::new();
    attest.record(AttestationRecord::new(
        "a1",
        "t1",
        "n1",
        AttestationStatus::Trusted,
        "q",
        0,
    ));
    attest.record(AttestationRecord::new(
        "a2",
        "t1",
        "n1",
        AttestationStatus::Untrusted,
        "q",
        0,
    ));
    let stats = BootStats::from(&chain, &attest);
    assert!((stats.trust_rate() - 0.5).abs() < f64::EPSILON);
}
#[test]
fn trust_rate_zero_with_no_attestations() {
    let chain = BootChain::new("t1", "n1");
    let attest = AttestationLog::new();
    let stats = BootStats::from(&chain, &attest);
    assert_eq!(stats.trust_rate(), 0.0);
}
