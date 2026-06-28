//! Tenant onboarding template.
//!
//! Generates the configuration and provisioning steps required to onboard
//! a new tenant into a multi-tenant Ancora deployment.

use std::collections::HashMap;

/// Tier of a tenant subscription.
#[derive(Debug, Clone, PartialEq)]
pub enum TenantTier {
    Free,
    Starter,
    Business,
    Enterprise,
}

impl TenantTier {
    pub fn as_str(&self) -> &'static str {
        match self {
            TenantTier::Free => "free",
            TenantTier::Starter => "starter",
            TenantTier::Business => "business",
            TenantTier::Enterprise => "enterprise",
        }
    }

    /// Returns the default resource quota for the tier.
    pub fn default_quota(&self) -> ResourceQuota {
        match self {
            TenantTier::Free => ResourceQuota { max_agents: 1, max_requests_per_minute: 10, max_storage_gb: 1 },
            TenantTier::Starter => ResourceQuota { max_agents: 5, max_requests_per_minute: 100, max_storage_gb: 10 },
            TenantTier::Business => ResourceQuota { max_agents: 50, max_requests_per_minute: 1_000, max_storage_gb: 100 },
            TenantTier::Enterprise => ResourceQuota { max_agents: u32::MAX, max_requests_per_minute: u32::MAX, max_storage_gb: u32::MAX },
        }
    }
}

/// Resource quota for a tenant.
#[derive(Debug, Clone, PartialEq)]
pub struct ResourceQuota {
    pub max_agents: u32,
    pub max_requests_per_minute: u32,
    pub max_storage_gb: u32,
}

/// Identity provider for tenant SSO.
#[derive(Debug, Clone, PartialEq)]
pub enum IdentityProvider {
    Internal,
    Saml(String),
    Oidc(String),
}

impl IdentityProvider {
    pub fn as_str(&self) -> String {
        match self {
            IdentityProvider::Internal => "internal".to_string(),
            IdentityProvider::Saml(url) => format!("saml:{}", url),
            IdentityProvider::Oidc(url) => format!("oidc:{}", url),
        }
    }
}

/// Configuration for onboarding a new tenant.
#[derive(Debug, Clone)]
pub struct TenantOnboardConfig {
    pub tenant_id: String,
    pub display_name: String,
    pub tier: TenantTier,
    pub admin_email: String,
    pub identity_provider: IdentityProvider,
    pub quota: ResourceQuota,
    pub namespace: String,
    pub extra_labels: HashMap<String, String>,
}

impl TenantOnboardConfig {
    pub fn new(
        tenant_id: impl Into<String>,
        display_name: impl Into<String>,
        tier: TenantTier,
        admin_email: impl Into<String>,
    ) -> Self {
        let tier_clone = tier.clone();
        let tenant_id_str = tenant_id.into();
        Self {
            namespace: format!("ancora-tenant-{}", tenant_id_str),
            tenant_id: tenant_id_str,
            display_name: display_name.into(),
            quota: tier_clone.default_quota(),
            tier,
            admin_email: admin_email.into(),
            identity_provider: IdentityProvider::Internal,
            extra_labels: HashMap::new(),
        }
    }

    pub fn with_idp(mut self, idp: IdentityProvider) -> Self {
        self.identity_provider = idp;
        self
    }

    pub fn with_quota(mut self, quota: ResourceQuota) -> Self {
        self.quota = quota;
        self
    }
}

/// Provisioning step for tenant onboarding.
#[derive(Debug, Clone)]
pub struct ProvisionStep {
    pub name: String,
    pub description: String,
    pub required: bool,
}

/// Rendered tenant onboarding template.
#[derive(Debug, Clone)]
pub struct TenantOnboardTemplate {
    pub config: TenantOnboardConfig,
    pub rendered: String,
    pub steps: Vec<ProvisionStep>,
}

impl TenantOnboardTemplate {
    pub fn render(config: TenantOnboardConfig) -> Result<Self, TenantError> {
        if config.tenant_id.is_empty() {
            return Err(TenantError::InvalidConfig("tenant_id is required".to_string()));
        }
        if config.admin_email.is_empty() {
            return Err(TenantError::InvalidConfig("admin_email is required".to_string()));
        }
        if !config.admin_email.contains('@') {
            return Err(TenantError::InvalidConfig("admin_email must be a valid email".to_string()));
        }

        let rendered = format!(
            "# ancora-pkg tenant onboarding template\n\
             tenant_id: {tenant}\n\
             display_name: {name}\n\
             tier: {tier}\n\
             admin_email: {email}\n\
             namespace: {ns}\n\
             identity_provider: {idp}\n\
             quota:\n\
             \x20\x20max_agents: {agents}\n\
             \x20\x20max_requests_per_minute: {rpm}\n\
             \x20\x20max_storage_gb: {storage}\n\
             security:\n\
             \x20\x20rbac_enabled: true\n\
             \x20\x20data_isolation: namespace\n\
             \x20\x20audit_log: enabled\n\
             \x20\x20mfa_required: true\n",
            tenant = config.tenant_id,
            name = config.display_name,
            tier = config.tier.as_str(),
            email = config.admin_email,
            ns = config.namespace,
            idp = config.identity_provider.as_str(),
            agents = config.quota.max_agents,
            rpm = config.quota.max_requests_per_minute,
            storage = config.quota.max_storage_gb,
        );

        let steps = vec![
            ProvisionStep {
                name: "create_namespace".to_string(),
                description: "Create isolated Kubernetes namespace for tenant".to_string(),
                required: true,
            },
            ProvisionStep {
                name: "apply_rbac".to_string(),
                description: "Apply RBAC roles and bindings".to_string(),
                required: true,
            },
            ProvisionStep {
                name: "configure_quotas".to_string(),
                description: "Set resource quota limits".to_string(),
                required: true,
            },
            ProvisionStep {
                name: "send_invite".to_string(),
                description: "Send onboarding email to admin".to_string(),
                required: false,
            },
        ];

        Ok(Self { config, rendered, steps })
    }

    pub fn contains(&self, field: &str) -> bool {
        self.rendered.contains(field)
    }

    pub fn required_step_count(&self) -> usize {
        self.steps.iter().filter(|s| s.required).count()
    }
}

/// Errors for tenant onboarding.
#[derive(Debug, Clone, PartialEq)]
pub enum TenantError {
    InvalidConfig(String),
}

impl std::fmt::Display for TenantError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TenantError::InvalidConfig(msg) => write!(f, "TenantError: {}", msg),
        }
    }
}

impl std::error::Error for TenantError {}
