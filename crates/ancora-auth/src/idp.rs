use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdpKind {
    Oidc,
    Saml,
}

#[derive(Debug, Clone)]
pub struct IdpConfig {
    pub tenant_id: String,
    pub kind: IdpKind,
    pub issuer: String,
    pub client_id: String,
    pub client_secret: String,
    pub scopes: Vec<String>,
    pub mfa_required: bool,
    pub extra: HashMap<String, String>,
}

impl IdpConfig {
    pub fn oidc(
        tenant_id: impl Into<String>,
        issuer: impl Into<String>,
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
    ) -> Self {
        Self {
            tenant_id: tenant_id.into(),
            kind: IdpKind::Oidc,
            issuer: issuer.into(),
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            scopes: vec!["openid".into(), "profile".into(), "email".into()],
            mfa_required: false,
            extra: HashMap::new(),
        }
    }

    pub fn saml(
        tenant_id: impl Into<String>,
        issuer: impl Into<String>,
        entity_id: impl Into<String>,
        acs_url: impl Into<String>,
    ) -> Self {
        let mut extra = HashMap::new();
        extra.insert("entity_id".into(), entity_id.into());
        extra.insert("acs_url".into(), acs_url.into());
        Self {
            tenant_id: tenant_id.into(),
            kind: IdpKind::Saml,
            issuer: issuer.into(),
            client_id: String::new(),
            client_secret: String::new(),
            scopes: Vec::new(),
            mfa_required: false,
            extra,
        }
    }

    pub fn with_mfa(mut self, required: bool) -> Self {
        self.mfa_required = required;
        self
    }

    pub fn with_extra(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.extra.insert(key.into(), value.into());
        self
    }
}

#[derive(Debug, Default)]
pub struct IdpRegistry {
    configs: HashMap<String, IdpConfig>,
}

impl IdpRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, config: IdpConfig) {
        self.configs.insert(config.tenant_id.clone(), config);
    }

    pub fn get(&self, tenant_id: &str) -> Option<&IdpConfig> {
        self.configs.get(tenant_id)
    }

    pub fn remove(&mut self, tenant_id: &str) -> Option<IdpConfig> {
        self.configs.remove(tenant_id)
    }

    pub fn tenant_ids(&self) -> Vec<&str> {
        self.configs.keys().map(String::as_str).collect()
    }
}
