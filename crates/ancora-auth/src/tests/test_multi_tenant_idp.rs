use crate::{IdpConfig, IdpKind, IdpRegistry};

#[test]
fn registry_holds_multiple_tenants() {
    let mut reg = IdpRegistry::new();
    reg.register(IdpConfig::oidc("tenant-a", "https://a.com", "ca", "sa"));
    reg.register(IdpConfig::saml(
        "tenant-b",
        "https://b.com",
        "urn:b",
        "https://b.com/acs",
    ));
    assert_eq!(reg.get("tenant-a").map(|c| &c.kind), Some(&IdpKind::Oidc));
    assert_eq!(reg.get("tenant-b").map(|c| &c.kind), Some(&IdpKind::Saml));
}

#[test]
fn registry_remove_tenant() {
    let mut reg = IdpRegistry::new();
    reg.register(IdpConfig::oidc("tenant-c", "https://c.com", "cc", "sc"));
    let removed = reg.remove("tenant-c");
    assert!(removed.is_some());
    assert!(reg.get("tenant-c").is_none());
}

#[test]
fn different_tenants_have_separate_configs() {
    let mut reg = IdpRegistry::new();
    reg.register(IdpConfig::oidc("alpha", "https://alpha.com", "c1", "s1").with_mfa(true));
    reg.register(IdpConfig::oidc("beta", "https://beta.com", "c2", "s2").with_mfa(false));
    assert!(reg.get("alpha").unwrap().mfa_required);
    assert!(!reg.get("beta").unwrap().mfa_required);
}
