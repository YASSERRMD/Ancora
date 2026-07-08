use crate::label::SensitivityLevel;

#[derive(Debug, Clone)]
pub struct ClassificationPolicy {
    pub tenant_id: String,
    pub max_allowed_level: SensitivityLevel,
    pub require_category_tag: bool,
    pub deny_public_write: bool,
}

impl ClassificationPolicy {
    pub fn new(tenant_id: impl Into<String>, max_allowed_level: SensitivityLevel) -> Self {
        Self {
            tenant_id: tenant_id.into(),
            max_allowed_level,
            require_category_tag: false,
            deny_public_write: false,
        }
    }

    pub fn strict(tenant_id: impl Into<String>) -> Self {
        Self {
            tenant_id: tenant_id.into(),
            max_allowed_level: SensitivityLevel::Confidential,
            require_category_tag: true,
            deny_public_write: true,
        }
    }

    pub fn permissive(tenant_id: impl Into<String>) -> Self {
        Self {
            tenant_id: tenant_id.into(),
            max_allowed_level: SensitivityLevel::TopSecret,
            require_category_tag: false,
            deny_public_write: false,
        }
    }

    pub fn with_require_category_tag(mut self) -> Self {
        self.require_category_tag = true;
        self
    }

    pub fn with_deny_public_write(mut self) -> Self {
        self.deny_public_write = true;
        self
    }
}

pub struct PolicyStore {
    policies: std::collections::HashMap<String, ClassificationPolicy>,
}

impl PolicyStore {
    pub fn new() -> Self {
        Self {
            policies: std::collections::HashMap::new(),
        }
    }

    pub fn insert(&mut self, policy: ClassificationPolicy) {
        self.policies.insert(policy.tenant_id.clone(), policy);
    }

    pub fn get(&self, tenant_id: &str) -> Option<&ClassificationPolicy> {
        self.policies.get(tenant_id)
    }

    pub fn count(&self) -> usize {
        self.policies.len()
    }
}

impl Default for PolicyStore {
    fn default() -> Self {
        Self::new()
    }
}
