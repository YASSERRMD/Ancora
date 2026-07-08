use crate::{IdpConfig, IdpKind, IdpRegistry};

#[test]
fn oidc_config_has_correct_kind() {
    let config = IdpConfig::oidc("t1", "https://iss.example.com", "client1", "secret");
    assert_eq!(config.kind, IdpKind::Oidc);
    assert_eq!(config.tenant_id, "t1");
    assert!(config.scopes.contains(&"openid".to_string()));
}

#[test]
fn saml_config_has_correct_kind() {
    let config = IdpConfig::saml(
        "t2",
        "https://saml.example.com",
        "urn:sp",
        "https://acs.example.com/acs",
    );
    assert_eq!(config.kind, IdpKind::Saml);
    assert_eq!(config.extra.get("entity_id"), Some(&"urn:sp".to_string()));
    assert_eq!(
        config.extra.get("acs_url"),
        Some(&"https://acs.example.com/acs".to_string())
    );
}

#[test]
fn idp_registry_register_and_get() {
    let mut reg = IdpRegistry::new();
    let config = IdpConfig::oidc("tenant-abc", "https://iss.example.com", "c1", "s1");
    reg.register(config);
    assert!(reg.get("tenant-abc").is_some());
    assert!(reg.get("unknown").is_none());
}

#[test]
fn idp_registry_tenant_ids_list() {
    let mut reg = IdpRegistry::new();
    reg.register(IdpConfig::oidc("t1", "https://a.com", "c", "s"));
    reg.register(IdpConfig::oidc("t2", "https://b.com", "c", "s"));
    let mut ids = reg.tenant_ids();
    ids.sort();
    assert_eq!(ids, vec!["t1", "t2"]);
}
