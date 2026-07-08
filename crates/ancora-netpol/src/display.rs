use crate::rule::{Effect, NetworkRule, Protocol};
use std::fmt;

impl fmt::Display for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Protocol::Tcp => write!(f, "TCP"),
            Protocol::Udp => write!(f, "UDP"),
            Protocol::Any => write!(f, "ANY"),
        }
    }
}

impl fmt::Display for Effect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Effect::Allow => write!(f, "ALLOW"),
            Effect::Deny => write!(f, "DENY"),
        }
    }
}

impl fmt::Display for NetworkRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {} {}:{} {} (priority={})",
            self.id,
            self.effect,
            self.host_pattern,
            self.port
                .map(|p| p.to_string())
                .unwrap_or_else(|| "*".to_string()),
            self.protocol,
            self.priority,
        )
    }
}
