use crate::idp::{IdpConfig, IdpKind};
use crate::token::{Token, TokenKind};

#[derive(Debug, PartialEq, Eq)]
pub enum SamlError {
    WrongIdpKind,
    AssertionInvalid,
    SignatureInvalid,
    SubjectMissing,
    MfaRequired,
    Expired,
}

#[derive(Debug, Clone)]
pub struct SamlAssertion {
    pub assertion_id: String,
    pub issuer: String,
    pub subject: String,
    pub tenant_id: String,
    pub attributes: std::collections::HashMap<String, String>,
    pub valid_from_tick: u64,
    pub valid_until_tick: u64,
    pub signed: bool,
    pub mfa_context: Option<String>,
}

impl SamlAssertion {
    pub fn new(
        assertion_id: impl Into<String>,
        issuer: impl Into<String>,
        subject: impl Into<String>,
        tenant_id: impl Into<String>,
        valid_from_tick: u64,
        valid_until_tick: u64,
    ) -> Self {
        Self {
            assertion_id: assertion_id.into(),
            issuer: issuer.into(),
            subject: subject.into(),
            tenant_id: tenant_id.into(),
            attributes: std::collections::HashMap::new(),
            valid_from_tick,
            valid_until_tick,
            signed: false,
            mfa_context: None,
        }
    }

    pub fn with_attribute(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.attributes.insert(key.into(), value.into());
        self
    }

    pub fn with_signed(mut self, signed: bool) -> Self {
        self.signed = signed;
        self
    }

    pub fn with_mfa_context(mut self, ctx: impl Into<String>) -> Self {
        self.mfa_context = Some(ctx.into());
        self
    }
}

pub struct MockSamlIdp {
    pub issuer: String,
    pub trusted_entity_ids: Vec<String>,
}

impl MockSamlIdp {
    pub fn new(issuer: impl Into<String>) -> Self {
        Self {
            issuer: issuer.into(),
            trusted_entity_ids: Vec::new(),
        }
    }

    pub fn trust_entity(mut self, entity_id: impl Into<String>) -> Self {
        self.trusted_entity_ids.push(entity_id.into());
        self
    }

    pub fn validate(
        &self,
        assertion: &SamlAssertion,
        config: &IdpConfig,
        current_tick: u64,
    ) -> Result<Token, SamlError> {
        if config.kind != IdpKind::Saml {
            return Err(SamlError::WrongIdpKind);
        }
        if !assertion.signed {
            return Err(SamlError::SignatureInvalid);
        }
        if assertion.subject.is_empty() {
            return Err(SamlError::SubjectMissing);
        }
        if current_tick >= assertion.valid_until_tick || current_tick < assertion.valid_from_tick {
            return Err(SamlError::Expired);
        }
        if config.mfa_required && assertion.mfa_context.is_none() {
            return Err(SamlError::MfaRequired);
        }
        let entity_id = config.extra.get("entity_id").map(String::as_str).unwrap_or("");
        if !self.trusted_entity_ids.is_empty() && !self.trusted_entity_ids.iter().any(|e| e == entity_id) {
            return Err(SamlError::AssertionInvalid);
        }
        Ok(Token::new(
            format!("saml:{}", assertion.assertion_id),
            TokenKind::Bearer,
            assertion.subject.clone(),
            assertion.tenant_id.clone(),
            assertion.valid_until_tick,
            Vec::new(),
        ))
    }
}
