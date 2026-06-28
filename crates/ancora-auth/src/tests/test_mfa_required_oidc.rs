use crate::{IdpConfig, JwkKey, JwtClaims, MockOidcIdp, OidcAuthCode, OidcError};

#[test]
fn oidc_mfa_required_blocks_unverified() {
    let mut idp = MockOidcIdp::new("https://idp.example.com", "ancora");
    idp.jwks.add_key(JwkKey::new("k1", "m", "AQAB", 0, 9999));
    let claims = JwtClaims::new("dave", "https://idp.example.com", "ancora", "tenant-mfa", 0, 500);
    idp.register_code("code-mfa", claims);
    let config = IdpConfig::oidc("tenant-mfa", "https://idp.example.com", "c", "s")
        .with_mfa(true);
    let auth_code = OidcAuthCode::new("code-mfa", "tenant-mfa", false);
    let err = idp.exchange(&auth_code, &config, "k1", 100).unwrap_err();
    assert_eq!(err, OidcError::MfaRequired);
}

#[test]
fn oidc_mfa_verified_succeeds() {
    let mut idp = MockOidcIdp::new("https://idp.example.com", "ancora");
    idp.jwks.add_key(JwkKey::new("k1", "m", "AQAB", 0, 9999));
    let claims = JwtClaims::new("dave", "https://idp.example.com", "ancora", "tenant-mfa", 0, 500);
    idp.register_code("code-mfa-ok", claims);
    let config = IdpConfig::oidc("tenant-mfa", "https://idp.example.com", "c", "s")
        .with_mfa(true);
    let auth_code = OidcAuthCode::new("code-mfa-ok", "tenant-mfa", true);
    let token = idp.exchange(&auth_code, &config, "k1", 100).expect("ok");
    assert_eq!(token.subject, "dave");
}
