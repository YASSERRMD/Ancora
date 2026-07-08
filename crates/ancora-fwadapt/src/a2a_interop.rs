/// Agent-to-Agent (A2A) interoperability with external frameworks.
///
/// Provides a protocol-neutral message envelope and a dispatcher that routes
/// messages between Ancora agents and external agents (simulated locally).
/// No network I/O is required - all external agents are represented by
/// in-process function pointers for testing.

#[derive(Debug, Clone, PartialEq)]
pub struct A2AMessage {
    pub sender_id: String,
    pub recipient_id: String,
    pub content: String,
    pub correlation_id: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct A2AResponse {
    pub responder_id: String,
    pub content: String,
    pub correlation_id: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum A2AError {
    UnknownRecipient(String),
    DispatchFailed(String),
}

impl std::fmt::Display for A2AError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownRecipient(id) => write!(f, "unknown recipient: {}", id),
            Self::DispatchFailed(r) => write!(f, "dispatch failed: {}", r),
        }
    }
}

pub type AgentHandler = fn(&A2AMessage) -> Result<A2AResponse, A2AError>;

/// A dispatcher that routes A2A messages to registered external agent handlers.
pub struct A2ADispatcher {
    handlers: Vec<(String, AgentHandler)>,
}

impl A2ADispatcher {
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
        }
    }

    /// Register an external agent handler.
    pub fn register(&mut self, agent_id: impl Into<String>, handler: AgentHandler) {
        self.handlers.push((agent_id.into(), handler));
    }

    /// Dispatch a message to the appropriate external agent.
    pub fn dispatch(&self, msg: &A2AMessage) -> Result<A2AResponse, A2AError> {
        for (id, handler) in &self.handlers {
            if *id == msg.recipient_id {
                return handler(msg);
            }
        }
        Err(A2AError::UnknownRecipient(msg.recipient_id.clone()))
    }

    pub fn registered_agents(&self) -> Vec<&str> {
        self.handlers.iter().map(|(id, _)| id.as_str()).collect()
    }
}

impl Default for A2ADispatcher {
    fn default() -> Self {
        Self::new()
    }
}

/// Build an A2A message with a generated correlation ID.
pub fn build_message(sender: &str, recipient: &str, content: &str) -> A2AMessage {
    A2AMessage {
        sender_id: sender.to_string(),
        recipient_id: recipient.to_string(),
        content: content.to_string(),
        correlation_id: format!("{}_{}", sender, recipient),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn echo_handler(msg: &A2AMessage) -> Result<A2AResponse, A2AError> {
        Ok(A2AResponse {
            responder_id: msg.recipient_id.clone(),
            content: format!("echo: {}", msg.content),
            correlation_id: msg.correlation_id.clone(),
        })
    }

    #[test]
    fn dispatch_to_known_agent() {
        let mut d = A2ADispatcher::new();
        d.register("ext-agent", echo_handler);
        let msg = build_message("ancora", "ext-agent", "hello");
        let resp = d.dispatch(&msg).unwrap();
        assert_eq!(resp.content, "echo: hello");
    }

    #[test]
    fn dispatch_unknown_returns_error() {
        let d = A2ADispatcher::new();
        let msg = build_message("ancora", "ghost", "hi");
        assert!(matches!(
            d.dispatch(&msg),
            Err(A2AError::UnknownRecipient(_))
        ));
    }
}
