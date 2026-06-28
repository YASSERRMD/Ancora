use ancora_auth::{IdpConfig, JwkKey, JwtClaims, MockOidcIdp, OidcAuthCode};

fn main() {
    let mut idp = MockOidcIdp::new("https://idp.example.com", "ancora");
    idp.jwks.add_key(JwkKey::new("key-2024", "modulus", "AQAB", 0, 100_000));

    let claims = JwtClaims::new("alice@example.com", "https://idp.example.com", "ancora", "acme-corp", 0, 3600)
        .with_scope("openid")
        .with_scope("email");
    idp.register_code("auth-code-xyz", claims);

    let config = IdpConfig::oidc("acme-corp", "https://idp.example.com", "client-id", "client-secret");
    let auth_code = OidcAuthCode::new("auth-code-xyz", "acme-corp", false);

    match idp.exchange(&auth_code, &config, "key-2024", 100) {
        Ok(token) => {
            println!("OIDC login ok: subject={}, tenant={}", token.subject, token.tenant_id);
            println!("  scopes: {:?}", token.scopes);
            println!("  expires_at_tick: {}", token.expires_at_tick);
        }
        Err(e) => eprintln!("OIDC error: {:?}", e),
    }
}
