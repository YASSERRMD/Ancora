use crate::{IdpConfig, MockSamlIdp, SamlAssertion, SamlError};

#[test]
fn saml_assertion_without_signature_rejected() {
    let idp = MockSamlIdp::new("https://idp.test").trust_entity("urn:sp:test");
    let config = IdpConfig::saml("t", "https://idp.test", "urn:sp:test", "https://acs.test");
    let assertion = SamlAssertion::new("no-sig", "https://idp.test", "frank", "t", 0, 500);
    let err = idp.validate(&assertion, &config, 100).unwrap_err();
    assert_eq!(err, SamlError::SignatureInvalid);
}

#[test]
fn saml_assertion_empty_subject_rejected() {
    let idp = MockSamlIdp::new("https://idp.test").trust_entity("urn:sp:test");
    let config = IdpConfig::saml("t", "https://idp.test", "urn:sp:test", "https://acs.test");
    let assertion = SamlAssertion::new("no-subj", "https://idp.test", "", "t", 0, 500)
        .with_signed(true);
    let err = idp.validate(&assertion, &config, 100).unwrap_err();
    assert_eq!(err, SamlError::SubjectMissing);
}
