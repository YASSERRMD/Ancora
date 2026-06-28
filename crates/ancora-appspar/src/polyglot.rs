/// Polyglot agent-to-agent (A2A) composition module.
///
/// Demonstrates composing agents implemented in different languages via
/// a simple in-process A2A routing table. No network calls are made.

use std::collections::HashMap;

/// A language identifier for an agent implementation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Language {
    Go,
    Python,
    TypeScript,
    DotNet,
    Java,
    Rust,
}

impl Language {
    pub fn as_str(&self) -> &'static str {
        match self {
            Language::Go => "go",
            Language::Python => "python",
            Language::TypeScript => "typescript",
            Language::DotNet => "dotnet",
            Language::Java => "java",
            Language::Rust => "rust",
        }
    }
}

/// A stub agent endpoint in the routing table.
#[derive(Debug, Clone)]
pub struct AgentEndpoint {
    pub language: Language,
    pub address: String,
}

/// A polyglot router that dispatches requests to language-specific agents.
#[derive(Debug)]
pub struct PolyglotRouter {
    routes: HashMap<Language, AgentEndpoint>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RouterError {
    NoEndpointForLanguage(String),
    EmptyPayload,
}

impl std::fmt::Display for RouterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RouterError::NoEndpointForLanguage(l) => {
                write!(f, "no endpoint registered for language: {}", l)
            }
            RouterError::EmptyPayload => write!(f, "payload must not be empty"),
        }
    }
}

/// A dispatched A2A message.
#[derive(Debug, Clone, PartialEq)]
pub struct A2aMessage {
    pub from_language: String,
    pub to_language: String,
    pub payload: String,
    pub trace_id: String,
}

impl PolyglotRouter {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    pub fn register(&mut self, language: Language, address: impl Into<String>) {
        let addr = address.into();
        let lang_clone = language.clone();
        self.routes.insert(
            language,
            AgentEndpoint {
                language: lang_clone,
                address: addr,
            },
        );
    }

    /// Dispatch a message from one language's agent to another.
    pub fn dispatch(
        &self,
        from: &Language,
        to: &Language,
        payload: &str,
    ) -> Result<A2aMessage, RouterError> {
        if payload.is_empty() {
            return Err(RouterError::EmptyPayload);
        }
        if !self.routes.contains_key(to) {
            return Err(RouterError::NoEndpointForLanguage(to.as_str().to_string()));
        }
        let trace_id = format!(
            "a2a-{}-{}-{}",
            from.as_str(),
            to.as_str(),
            payload.len()
        );
        Ok(A2aMessage {
            from_language: from.as_str().to_string(),
            to_language: to.as_str().to_string(),
            payload: payload.to_string(),
            trace_id,
        })
    }

    pub fn registered_languages(&self) -> Vec<&Language> {
        let mut langs: Vec<&Language> = self.routes.keys().collect();
        langs.sort_by_key(|l| l.as_str());
        langs
    }
}

impl Default for PolyglotRouter {
    fn default() -> Self {
        Self::new()
    }
}

/// Build a fully-connected router with all six languages registered.
pub fn full_router() -> PolyglotRouter {
    let mut router = PolyglotRouter::new();
    router.register(Language::Go, "in-process://go");
    router.register(Language::Python, "in-process://python");
    router.register(Language::TypeScript, "in-process://typescript");
    router.register(Language::DotNet, "in-process://dotnet");
    router.register(Language::Java, "in-process://java");
    router.register(Language::Rust, "in-process://rust");
    router
}
