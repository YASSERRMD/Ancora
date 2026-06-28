use crate::Tenant;
#[test]
fn metadata_stored_via_builder() {
    let t = Tenant::new("t1", "Acme", 1)
        .with_metadata("plan", "enterprise")
        .with_metadata("region", "us-east-1");
    assert_eq!(t.metadata.get("plan").unwrap(), "enterprise");
    assert_eq!(t.metadata.get("region").unwrap(), "us-east-1");
}
