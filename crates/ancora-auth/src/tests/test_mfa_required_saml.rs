use crate::{IdpConfig, MockSamlIdp, SamlAssertion, SamlError};

#[test]
fn saml_mfa_required_blocks_without_context() {
    let idp = MockSamlIdp::new("https://saml-idp.example.com")
        .trust_entity("urn:sp");
    let config = IdpConfig::saml("tenant-saml-mfa", "https://saml-idp.example.com", "urn:sp", "https://acs")
        .with_mfa(true);
    let assertion = SamlAssertion::new("a1", "https://saml-idp.example.com", "eve", "tenant-saml-mfa", 0, 500)
        .with_signed(true);
    let err = idp.validate(&assertion, &config, 100).unwrap_err();
    assert_eq!(err, SamlError::MfaRequired);
}

#[test]
fn saml_mfa_with_context_succeeds() {
    let idp = MockSamlIdp::new("https://saml-idp.example.com")
        .trust_entity("urn:sp");
    let config = IdpConfig::saml("tenant-saml-mfa", "https://saml-idp.example.com", "urn:sp", "https://acs")
        .with_mfa(true);
    let assertion = SamlAssertion::new("a2", "https://saml-idp.example.com", "eve", "tenant-saml-mfa", 0, 500)
        .with_signed(true)
        .with_mfa_context("PasswordProtectedTransport");
    let token = idp.validate(&assertion, &config, 100).expect("ok");
    assert_eq!(token.subject, "eve");
}
