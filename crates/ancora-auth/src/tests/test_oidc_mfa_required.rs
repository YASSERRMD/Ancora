use crate::{IdpConfig, JwkKey, JwtClaims, MockOidcIdp, OidcAuthCode, OidcError};

#[test]
fn oidc_wrong_idp_kind_rejected() {
    let mut idp = MockOidcIdp::new("https://idp.example.com", "ancora");
    idp.jwks.add_key(JwkKey::new("k1", "m", "AQAB", 0, 9999));
    let claims = JwtClaims::new("henry", "https://idp.example.com", "ancora", "t", 0, 500);
    idp.register_code("code-h", claims);
    let config = IdpConfig::saml("t", "https://idp.example.com", "urn:sp", "https://acs");
    let auth_code = OidcAuthCode::new("code-h", "t", false);
    let err = idp.exchange(&auth_code, &config, "k1", 100).unwrap_err();
    assert_eq!(err, OidcError::WrongIdpKind);
}

#[test]
fn oidc_scopes_present_in_returned_token() {
    let mut idp = MockOidcIdp::new("https://idp.example.com", "ancora");
    idp.jwks.add_key(JwkKey::new("k1", "m", "AQAB", 0, 9999));
    let claims = JwtClaims::new("ivan", "https://idp.example.com", "ancora", "t", 0, 500)
        .with_scope("openid")
        .with_scope("email");
    idp.register_code("code-i", claims);
    let config = IdpConfig::oidc("t", "https://idp.example.com", "c", "s");
    let auth_code = OidcAuthCode::new("code-i", "t", false);
    let token = idp.exchange(&auth_code, &config, "k1", 100).expect("ok");
    assert!(token.has_scope("openid"));
    assert!(token.has_scope("email"));
}
