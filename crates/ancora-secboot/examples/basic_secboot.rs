use ancora_secboot::{
    AttestationLog, AttestationRecord, AttestationStatus,
    BootAuditEntry, BootAuditLog, BootChain, BootEvent,
    IntegrityEvaluator, MeasurementBuilder, MeasurementKind,
    SealingStore, IntegrityReport, BootStats,
    strict_boot_policy,
};

fn main() {
    let policy = strict_boot_policy("tenant-acme")
        .allow_digest("uefi.bin", "fw-digest-v2")
        .allow_digest("grub.efi", "boot-digest-v1")
        .allow_digest("vmlinuz", "kernel-digest-v5");

    let mut chain = BootChain::new("tenant-acme", "node-prod-01");
    chain.add_step(
        MeasurementBuilder::new("m1", "uefi.bin")
            .kind(MeasurementKind::Firmware)
            .digest("fw-digest-v2")
            .tick(100)
            .build(),
    );
    chain.add_step(
        MeasurementBuilder::new("m2", "grub.efi")
            .kind(MeasurementKind::Bootloader)
            .digest("boot-digest-v1")
            .tick(200)
            .build(),
    );
    chain.add_step(
        MeasurementBuilder::new("m3", "vmlinuz")
            .kind(MeasurementKind::Kernel)
            .digest("kernel-digest-v5")
            .tick(300)
            .build(),
    );

    let decision = IntegrityEvaluator::evaluate(&policy, &chain);
    println!("Integrity decision: {:?}", decision);

    let mut attest_log = AttestationLog::new();
    attest_log.record(AttestationRecord::new(
        "att-1", "tenant-acme", "node-prod-01",
        AttestationStatus::Trusted, "tpm-quote-abc", 400,
    ));

    let report = IntegrityReport::generate(&policy, &chain, &attest_log, 400);
    println!("Fully trusted: {}", report.is_fully_trusted());
    println!("Chain length: {}", report.chain_length);
    println!("Trusted attestations: {}", report.trusted_attestations);

    let stats = BootStats::from(&chain, &attest_log);
    println!("Trust rate: {:.0}%", stats.trust_rate() * 100.0);

    let mut sealing = SealingStore::new();
    sealing.seal("disk-key", "tenant-acme", "secret-disk-encryption-key", "kernel-digest-v5", 300);
    let unsealed = sealing.unseal("disk-key", "kernel-digest-v5");
    println!("Unseal result: {:?}", unsealed);

    let mut audit = BootAuditLog::new();
    audit.record(BootAuditEntry::new(400, "tenant-acme", "node-prod-01", BootEvent::ChainValidated, "system", true, "all checks passed"));
    println!("Audit entries: {}", audit.count());
    println!("done");
}
