use crate::secret::SecretVersion;
#[test]
fn version_with_metadata_stores_key_value() {
    let v = SecretVersion::new(1, "secret_value", 10).with_metadata("source", "hsm-slot-3");
    assert_eq!(v.metadata.get("source").unwrap(), "hsm-slot-3");
}
