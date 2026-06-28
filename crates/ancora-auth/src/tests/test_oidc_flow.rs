use crate::{IdpConfig, JwkKey, JwtClaims, JwksStore, MockOidcIdp, OidcAuthCode, OidcError};

fn make_oidc_setup() -> (MockOidcIdp, IdpConfig, String) {
    let mut idp = MockOidcIdp::new("https://idp.example.com", "ancora");
    let key = JwkKey::new("key1", "modulus", "AQAB", 0, 9999);
    idp.jwks.add_key(key);
    let claims = JwtClaims::new("alice", "https://idp.example.com", "ancora", "tenant-a", 0, 500)
        .with_scope("openid");
    idp.register_code("code-abc", claims);
    let config = IdpConfig::oidc("tenant-a", "https://idp.example.com", "client1", "secret");
    (idp, config, "key1".into())
}

#[test]
fn oidc_valid_code_returns_token() {
    let (idp, config, kid) = make_oidc_setup();
    let auth_code = OidcAuthCode::new("code-abc", "tenant-a", false);
    let token = idp.exchange(&auth_code, &config, &kid, 100).expect("valid exchange");
    assert_eq!(token.subject, "alice");
    assert_eq!(token.tenant_id, "tenant-a");
}

#[test]
fn oidc_invalid_code_rejected() {
    let (idp, config, kid) = make_oidc_setup();
    let auth_code = OidcAuthCode::new("bad-code", "tenant-a", false);
    let err = idp.exchange(&auth_code, &config, &kid, 100).unwrap_err();
    assert_eq!(err, OidcError::AuthCodeInvalid);
}

#[test]
fn oidc_expired_token_rejected() {
    let (idp, config, kid) = make_oidc_setup();
    let auth_code = OidcAuthCode::new("code-abc", "tenant-a", false);
    let err = idp.exchange(&auth_code, &config, &kid, 600).unwrap_err();
    assert!(matches!(err, OidcError::ValidationFailed(_)));
}
