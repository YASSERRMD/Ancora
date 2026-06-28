use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct AccessRequest {
    pub id: String,
    pub tenant_id: String,
    pub identity_id: String,
    pub device_id: Option<String>,
    pub resource: String,
    pub action: String,
    pub context: HashMap<String, String>,
    pub tick: u64,
}

impl AccessRequest {
    pub fn new(
        id: impl Into<String>,
        tenant_id: impl Into<String>,
        identity_id: impl Into<String>,
        resource: impl Into<String>,
        action: impl Into<String>,
        tick: u64,
    ) -> Self {
        Self {
            id: id.into(),
            tenant_id: tenant_id.into(),
            identity_id: identity_id.into(),
            device_id: None,
            resource: resource.into(),
            action: action.into(),
            context: HashMap::new(),
            tick,
        }
    }

    pub fn with_device(mut self, device_id: impl Into<String>) -> Self {
        self.device_id = Some(device_id.into()); self
    }

    pub fn with_context(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.context.insert(key.into(), value.into()); self
    }
}
