use crate::idp::{IdpConfig, IdpKind};
use crate::jwks::JwksStore;
use crate::jwt::{JwtClaims, JwtValidator};
use crate::token::Token;

#[derive(Debug, PartialEq, Eq)]
pub enum OidcError {
    WrongIdpKind,
    MfaRequired,
    AuthCodeInvalid,
    TokenExchangeFailed(String),
    ValidationFailed(String),
}

pub struct OidcAuthCode {
    pub code: String,
    pub tenant_id: String,
    pub mfa_verified: bool,
}

impl OidcAuthCode {
    pub fn new(code: impl Into<String>, tenant_id: impl Into<String>, mfa_verified: bool) -> Self {
        Self {
            code: code.into(),
            tenant_id: tenant_id.into(),
            mfa_verified,
        }
    }
}

pub struct MockOidcIdp {
    pub jwks: JwksStore,
    pub issuer: String,
    pub audience: String,
    pub valid_codes: std::collections::HashMap<String, JwtClaims>,
}

impl MockOidcIdp {
    pub fn new(issuer: impl Into<String>, audience: impl Into<String>) -> Self {
        Self {
            jwks: JwksStore::new(),
            issuer: issuer.into(),
            audience: audience.into(),
            valid_codes: std::collections::HashMap::new(),
        }
    }

    pub fn register_code(&mut self, code: impl Into<String>, claims: JwtClaims) {
        self.valid_codes.insert(code.into(), claims);
    }

    pub fn exchange(
        &self,
        auth_code: &OidcAuthCode,
        config: &IdpConfig,
        kid: &str,
        current_tick: u64,
    ) -> Result<Token, OidcError> {
        if config.kind != IdpKind::Oidc {
            return Err(OidcError::WrongIdpKind);
        }
        if config.mfa_required && !auth_code.mfa_verified {
            return Err(OidcError::MfaRequired);
        }
        let claims = self
            .valid_codes
            .get(&auth_code.code)
            .ok_or(OidcError::AuthCodeInvalid)?;
        let validator = JwtValidator::new(&self.jwks, &self.issuer, &self.audience);
        validator
            .validate(kid, claims, current_tick)
            .map_err(|e| OidcError::ValidationFailed(format!("{:?}", e)))
    }
}
