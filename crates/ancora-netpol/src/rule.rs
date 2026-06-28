#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Protocol {
    Tcp,
    Udp,
    Any,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Effect {
    Allow,
    Deny,
}

#[derive(Debug, Clone)]
pub struct NetworkRule {
    pub id: String,
    pub host_pattern: String,
    pub port: Option<u16>,
    pub protocol: Protocol,
    pub effect: Effect,
    pub priority: u32,
    pub description: String,
}

impl NetworkRule {
    pub fn new(
        id: impl Into<String>,
        host_pattern: impl Into<String>,
        port: Option<u16>,
        protocol: Protocol,
        effect: Effect,
        priority: u32,
    ) -> Self {
        Self {
            id: id.into(),
            host_pattern: host_pattern.into(),
            port,
            protocol,
            effect,
            priority,
            description: String::new(),
        }
    }

    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }

    pub fn matches_host(&self, host: &str) -> bool {
        if self.host_pattern == "*" { return true; }
        if self.host_pattern.starts_with("*.") {
            let suffix = &self.host_pattern[1..];
            return host.ends_with(suffix) || host == &self.host_pattern[2..];
        }
        self.host_pattern == host
    }

    pub fn matches_port(&self, port: u16) -> bool {
        self.port.map_or(true, |p| p == port)
    }

    pub fn matches_protocol(&self, proto: &Protocol) -> bool {
        self.protocol == Protocol::Any || &self.protocol == proto
    }

    pub fn matches(&self, host: &str, port: u16, proto: &Protocol) -> bool {
        self.matches_host(host) && self.matches_port(port) && self.matches_protocol(proto)
    }
}
