use ancora_auth::{IdpConfig, MockSamlIdp, SamlAssertion};

fn main() {
    let idp = MockSamlIdp::new("https://saml-idp.example.com")
        .trust_entity("urn:ancora:sp");

    let config = IdpConfig::saml(
        "gov-tenant",
        "https://saml-idp.example.com",
        "urn:ancora:sp",
        "https://ancora.example.com/acs",
    );

    let assertion = SamlAssertion::new(
        "assert-001",
        "https://saml-idp.example.com",
        "bob@gov.example.com",
        "gov-tenant",
        0,
        7200,
    )
    .with_signed(true)
    .with_attribute("department", "IT Security")
    .with_attribute("clearance", "SECRET");

    match idp.validate(&assertion, &config, 100) {
        Ok(token) => {
            println!("SAML login ok: subject={}, tenant={}", token.subject, token.tenant_id);
        }
        Err(e) => eprintln!("SAML error: {:?}", e),
    }
}
