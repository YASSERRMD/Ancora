use ancora_supplychain::{
    ComponentBuilder, ComponentKind, ComponentQuery, ComponentSignature, License, ProvenanceKind,
    ProvenanceRecord, ProvenanceStore, Sbom, SbomFormat, SbomStats, SignatureAlgorithm,
    SignatureStore, SupplyChainAuditEntry, SupplyChainAuditLog, SupplyChainEvent,
    SupplyChainPolicy, SupplyChainReport,
};

fn main() {
    let mut sbom = Sbom::new("sbom-prod-v1", "tenant-acme", SbomFormat::CycloneDx, 1000);

    let openssl = ComponentBuilder::new("openssl-3.0", "openssl", "3.0.7")
        .kind(ComponentKind::Library)
        .license(License::Apache2)
        .supplier("openssl-project")
        .digest("sha256:aabbcc")
        .build();

    let custom_lib = ComponentBuilder::new("acme-auth", "acme-auth", "2.1.0")
        .kind(ComponentKind::Library)
        .license(License::Proprietary)
        .supplier("acme-corp")
        .digest("sha256:ddeeff")
        .build();

    sbom.add_component(openssl);
    sbom.add_component(custom_lib);

    let mut sigs = SignatureStore::new();
    sigs.register(ComponentSignature::new(
        "openssl-3.0",
        SignatureAlgorithm::Ed25519,
        "ci-bot",
        "sig-abc",
        1001,
    ));
    sigs.register(ComponentSignature::new(
        "acme-auth",
        SignatureAlgorithm::Ed25519,
        "ci-bot",
        "sig-def",
        1001,
    ));

    let mut prov = ProvenanceStore::new();
    prov.record(ProvenanceRecord::new(
        "openssl-3.0",
        ProvenanceKind::Registry,
        "crates.io",
        "build-1",
        1000,
    ));
    prov.record(ProvenanceRecord::new(
        "acme-auth",
        ProvenanceKind::BuildSystem,
        "jenkins",
        "build-42",
        1000,
    ));

    let policy = SupplyChainPolicy::new("tenant-acme").require_signature();

    let report = SupplyChainReport::generate(&sbom, &sigs, &prov, &policy, 1010);
    println!("Total components: {}", report.total_components);
    println!("Signed: {}", report.signed_count);
    println!("Compliant: {}", report.is_compliant());
    println!("Sign rate: {:.0}%", report.sign_rate() * 100.0);

    let stats = SbomStats::from(&sbom);
    println!("OSS rate: {:.0}%", stats.oss_rate() * 100.0);
    println!(
        "Libraries: {}",
        stats.by_kind.get("LIBRARY").copied().unwrap_or(0)
    );

    let oss = ComponentQuery::new()
        .open_source_only()
        .run(sbom.components.iter());
    println!("Open source components: {}", oss.len());

    let mut audit = SupplyChainAuditLog::new();
    audit.record(SupplyChainAuditEntry::new(
        1001,
        "tenant-acme",
        "openssl-3.0",
        SupplyChainEvent::ComponentSigned,
        "ci-bot",
        true,
    ));
    audit.record(SupplyChainAuditEntry::new(
        1002,
        "tenant-acme",
        "acme-auth",
        SupplyChainEvent::ComponentSigned,
        "ci-bot",
        true,
    ));
    println!("Audit entries: {}", audit.count());
    println!("done");
}
