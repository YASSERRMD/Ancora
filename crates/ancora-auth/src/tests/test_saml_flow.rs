use crate::{IdpConfig, MockSamlIdp, SamlAssertion, SamlError};

fn make_saml_setup() -> (MockSamlIdp, IdpConfig) {
    let idp = MockSamlIdp::new("https://saml-idp.example.com").trust_entity("urn:ancora:sp");
    let config = IdpConfig::saml(
        "tenant-b",
        "https://saml-idp.example.com",
        "urn:ancora:sp",
        "https://ancora.example.com/acs",
    );
    (idp, config)
}

#[test]
fn saml_valid_assertion_returns_token() {
    let (idp, config) = make_saml_setup();
    let assertion = SamlAssertion::new(
        "assert-1",
        "https://saml-idp.example.com",
        "bob",
        "tenant-b",
        0,
        500,
    )
    .with_signed(true);
    let token = idp.validate(&assertion, &config, 100).expect("valid");
    assert_eq!(token.subject, "bob");
    assert_eq!(token.tenant_id, "tenant-b");
}

#[test]
fn saml_unsigned_assertion_rejected() {
    let (idp, config) = make_saml_setup();
    let assertion = SamlAssertion::new(
        "assert-2",
        "https://saml-idp.example.com",
        "bob",
        "tenant-b",
        0,
        500,
    );
    let err = idp.validate(&assertion, &config, 100).unwrap_err();
    assert_eq!(err, SamlError::SignatureInvalid);
}

#[test]
fn saml_expired_assertion_rejected() {
    let (idp, config) = make_saml_setup();
    let assertion = SamlAssertion::new(
        "assert-3",
        "https://saml-idp.example.com",
        "bob",
        "tenant-b",
        0,
        50,
    )
    .with_signed(true);
    let err = idp.validate(&assertion, &config, 100).unwrap_err();
    assert_eq!(err, SamlError::Expired);
}
