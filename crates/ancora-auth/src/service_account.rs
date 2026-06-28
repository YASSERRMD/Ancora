use crate::token::{Token, TokenKind};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ServiceAccount {
    pub account_id: String,
    pub tenant_id: String,
    pub key_hash: String,
    pub scopes: Vec<String>,
    pub enabled: bool,
    pub description: String,
}

impl ServiceAccount {
    pub fn new(
        account_id: impl Into<String>,
        tenant_id: impl Into<String>,
        key_hash: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            account_id: account_id.into(),
            tenant_id: tenant_id.into(),
            key_hash: key_hash.into(),
            scopes: Vec::new(),
            enabled: true,
            description: description.into(),
        }
    }

    pub fn with_scope(mut self, scope: impl Into<String>) -> Self {
        self.scopes.push(scope.into());
        self
    }

    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ServiceAccountError {
    NotFound,
    Disabled,
    InvalidKey,
    TokenMintFailed,
}

#[derive(Debug, Default)]
pub struct ServiceAccountRegistry {
    accounts: HashMap<String, ServiceAccount>,
}

impl ServiceAccountRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, account: ServiceAccount) {
        self.accounts.insert(account.account_id.clone(), account);
    }

    pub fn authenticate(
        &self,
        account_id: &str,
        key_hash: &str,
        ttl_ticks: u64,
        current_tick: u64,
    ) -> Result<Token, ServiceAccountError> {
        let account = self.accounts.get(account_id).ok_or(ServiceAccountError::NotFound)?;
        if !account.enabled {
            return Err(ServiceAccountError::Disabled);
        }
        if account.key_hash != key_hash {
            return Err(ServiceAccountError::InvalidKey);
        }
        Ok(Token::new(
            format!("sa:{account_id}:{current_tick}"),
            TokenKind::ServiceAccount,
            account_id,
            account.tenant_id.clone(),
            current_tick + ttl_ticks,
            account.scopes.clone(),
        ))
    }

    pub fn disable(&mut self, account_id: &str) -> bool {
        match self.accounts.get_mut(account_id) {
            Some(a) => {
                a.enabled = false;
                true
            }
            None => false,
        }
    }
}
