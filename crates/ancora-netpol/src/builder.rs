use crate::rule::{Effect, NetworkRule, Protocol};

pub struct RuleBuilder {
    id: String,
    host_pattern: String,
    port: Option<u16>,
    protocol: Protocol,
    effect: Effect,
    priority: u32,
    description: String,
}

impl RuleBuilder {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            host_pattern: "*".to_string(),
            port: None,
            protocol: Protocol::Any,
            effect: Effect::Allow,
            priority: 100,
            description: String::new(),
        }
    }

    pub fn host(mut self, pattern: impl Into<String>) -> Self {
        self.host_pattern = pattern.into();
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    pub fn tcp(mut self) -> Self {
        self.protocol = Protocol::Tcp;
        self
    }

    pub fn udp(mut self) -> Self {
        self.protocol = Protocol::Udp;
        self
    }

    pub fn any_protocol(mut self) -> Self {
        self.protocol = Protocol::Any;
        self
    }

    pub fn allow(mut self) -> Self {
        self.effect = Effect::Allow;
        self
    }

    pub fn deny(mut self) -> Self {
        self.effect = Effect::Deny;
        self
    }

    pub fn priority(mut self, priority: u32) -> Self {
        self.priority = priority;
        self
    }

    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }

    pub fn build(self) -> NetworkRule {
        NetworkRule::new(
            self.id,
            self.host_pattern,
            self.port,
            self.protocol,
            self.effect,
            self.priority,
        )
        .with_description(self.description)
    }
}
