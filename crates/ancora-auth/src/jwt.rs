use crate::token::{Token, TokenKind};
use crate::jwks::JwksStore;

#[derive(Debug, PartialEq, Eq)]
pub enum JwtError {
    MalformedHeader,
    MissingKid,
    UnknownKid(String),
    KeyExpired,
    TokenExpired,
    InvalidClaims(String),
}

#[derive(Debug, Clone)]
pub struct JwtClaims {
    pub sub: String,
    pub iss: String,
    pub aud: String,
    pub exp_tick: u64,
    pub iat_tick: u64,
    pub scopes: Vec<String>,
    pub tenant_id: String,
}

impl JwtClaims {
    pub fn new(
        sub: impl Into<String>,
        iss: impl Into<String>,
        aud: impl Into<String>,
        tenant_id: impl Into<String>,
        iat_tick: u64,
        exp_tick: u64,
    ) -> Self {
        Self {
            sub: sub.into(),
            iss: iss.into(),
            aud: aud.into(),
            exp_tick,
            iat_tick,
            scopes: Vec::new(),
            tenant_id: tenant_id.into(),
        }
    }

    pub fn with_scope(mut self, scope: impl Into<String>) -> Self {
        self.scopes.push(scope.into());
        self
    }

    pub fn encode(&self) -> String {
        format!(
            "mock.{}.{}",
            self.sub,
            self.exp_tick,
        )
    }
}

pub struct JwtValidator<'a> {
    store: &'a JwksStore,
    audience: String,
    issuer: String,
}

impl<'a> JwtValidator<'a> {
    pub fn new(store: &'a JwksStore, issuer: impl Into<String>, audience: impl Into<String>) -> Self {
        Self {
            store,
            issuer: issuer.into(),
            audience: audience.into(),
        }
    }

    pub fn validate(&self, kid: &str, claims: &JwtClaims, current_tick: u64) -> Result<Token, JwtError> {
        let key = self.store.get_key(kid).ok_or_else(|| JwtError::UnknownKid(kid.to_string()))?;
        if !key.is_active(current_tick) {
            return Err(JwtError::KeyExpired);
        }
        if claims.exp_tick <= current_tick {
            return Err(JwtError::TokenExpired);
        }
        if claims.iss != self.issuer {
            return Err(JwtError::InvalidClaims(format!("issuer mismatch: {}", claims.iss)));
        }
        if claims.aud != self.audience {
            return Err(JwtError::InvalidClaims(format!("audience mismatch: {}", claims.aud)));
        }
        Ok(Token::new(
            claims.encode(),
            TokenKind::Bearer,
            claims.sub.clone(),
            claims.tenant_id.clone(),
            claims.exp_tick,
            claims.scopes.clone(),
        ))
    }
}
