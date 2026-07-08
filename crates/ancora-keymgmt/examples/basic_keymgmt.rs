use ancora_keymgmt::{
    aes256_encryption_key, ed25519_signing_key, ephemeral_key, rotate_key, ExpiryChecker,
    HsmConfig, HsmStub, KeyAlgorithm, KeyAuditEntry, KeyAuditLog, KeyBuilder, KeyOperation,
    KeyPurpose, KeyStats, KeyStore, KeyValidator, RotationPolicy,
};

fn main() {
    let mut store = KeyStore::new();
    let mut audit = KeyAuditLog::new();

    let enc = aes256_encryption_key("enc-key-1", "tenant-acme", 100);
    let sig = ed25519_signing_key("sig-key-1", "tenant-acme", 100);
    let ephem = ephemeral_key("ephem-key-1", "tenant-acme", 100, 500);

    store.create(enc).unwrap();
    store.create(sig).unwrap();
    store.create(ephem).unwrap();

    audit.record(KeyAuditEntry::new(
        100,
        "tenant-acme",
        "enc-key-1",
        1,
        KeyOperation::Create,
        "admin",
        true,
    ));
    audit.record(KeyAuditEntry::new(
        101,
        "tenant-acme",
        "sig-key-1",
        1,
        KeyOperation::Create,
        "admin",
        true,
    ));

    let policy = RotationPolicy::new(5).with_rotation_interval(1000);
    let enc_key = store.get_active("tenant-acme", "enc-key-1").unwrap();
    println!(
        "Should rotate at tick 1100: {}",
        policy.should_rotate(enc_key, 1100)
    );

    let new_ver = rotate_key(
        &mut store,
        "tenant-acme",
        "enc-key-1",
        "new-material-v2",
        1100,
    )
    .unwrap();
    println!("Rotated enc-key-1 to version {}", new_ver);
    audit.record(KeyAuditEntry::new(
        1100,
        "tenant-acme",
        "enc-key-1",
        new_ver,
        KeyOperation::Rotate,
        "admin",
        true,
    ));

    let stats = KeyStats::for_tenant(&store, "tenant-acme");
    println!("Active keys for tenant-acme: {}", stats.total_active);
    println!("Algorithm distribution: {:?}", stats.by_algorithm);

    let expired = ExpiryChecker::expired_keys(&store, "tenant-acme", 700);
    println!("Expired keys at tick 700: {}", expired.len());

    let soon = ExpiryChecker::expiring_soon(&store, "tenant-acme", 500, 100);
    println!("Expiring within 100 ticks of tick 500: {}", soon.len());

    let active = store.get_active("tenant-acme", "enc-key-1").unwrap();
    let issues = KeyValidator::validate_key(active, 1100);
    println!("Validation issues: {}", issues.len());

    let mut hsm = HsmStub::new(HsmConfig::software());
    let hsm_key = hsm.generate_key(
        "hsm-key-1",
        "tenant-acme",
        KeyAlgorithm::EcdsaP256,
        KeyPurpose::Signing,
        1200,
    );
    println!("HSM key material prefix: {}", &hsm_key.key_material[..8]);

    let manual = KeyBuilder::new("manual-key-1", "tenant-acme")
        .algorithm(KeyAlgorithm::Hmac256)
        .purpose(KeyPurpose::Authentication)
        .tick(1300)
        .material("manual-secret")
        .build();
    store.create(manual).unwrap();

    println!(
        "Rotations for enc-key-1: {}",
        audit.rotations_for("enc-key-1").len()
    );
    println!("Total audit entries: {}", audit.count());
    println!("done");
}
