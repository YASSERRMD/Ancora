use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

/// Capability advertised in an A2A agent card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentCapability {
    Run,
    Stream,
    Resume,
    Tools,
}

/// Metadata describing an Ancora agent accessible to external callers.
/// Serialises as the JSON body returned by `GET /.well-known/agent.json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentCard {
    pub name: String,
    pub description: String,
    pub version: String,
    pub endpoint: String,
    pub capabilities: Vec<AgentCapability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

impl AgentCard {
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        endpoint: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            version: env!("CARGO_PKG_VERSION").to_owned(),
            endpoint: endpoint.into(),
            capabilities: vec![
                AgentCapability::Run,
                AgentCapability::Stream,
                AgentCapability::Resume,
            ],
            identity_key: None,
            signature: None,
        }
    }

    /// Serialise the card to a pretty-printed JSON string.
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("AgentCard is always serialisable")
    }

    /// Serialise the card without the `signature` field for signing or verification.
    pub(crate) fn canonical_bytes(&self) -> Vec<u8> {
        let mut c = self.clone();
        c.signature = None;
        serde_json::to_vec(&c).expect("AgentCard is always serialisable")
    }

    /// Serve this card over plain HTTP at `/.well-known/agent.json`.
    /// Runs until `shutdown` fires or the listener fails.
    pub async fn serve(
        self,
        addr: std::net::SocketAddr,
        shutdown: tokio::sync::oneshot::Receiver<()>,
    ) -> tokio::io::Result<()> {
        let listener = tokio::net::TcpListener::bind(addr).await?;
        let body = Arc::new(self.to_json());
        tokio::select! {
            _ = async {
                loop {
                    let (mut stream, _) = match listener.accept().await {
                        Ok(s) => s,
                        Err(_) => break,
                    };
                    let body = Arc::clone(&body);
                    tokio::spawn(async move {
                        let mut buf = [0u8; 2048];
                        let n = stream.read(&mut buf).await.unwrap_or(0);
                        let ok = buf[..n]
                            .windows(b"agent.json".len())
                            .any(|w| w == b"agent.json");
                        let response = if ok {
                            format!(
                                "HTTP/1.1 200 OK\r\n\
                                 Content-Type: application/json\r\n\
                                 Content-Length: {}\r\n\r\n{}",
                                body.len(),
                                &*body
                            )
                        } else {
                            "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n".into()
                        };
                        stream.write_all(response.as_bytes()).await.ok();
                    });
                }
            } => {}
            _ = shutdown => {}
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_card_has_default_capabilities() {
        let card = AgentCard::new("agent", "desc", "grpc://localhost:50051");
        assert!(card.capabilities.contains(&AgentCapability::Run));
        assert!(card.capabilities.contains(&AgentCapability::Stream));
        assert!(card.capabilities.contains(&AgentCapability::Resume));
        assert!(card.identity_key.is_none());
        assert!(card.signature.is_none());
    }

    #[test]
    fn to_json_contains_name_and_endpoint() {
        let card = AgentCard::new("my-agent", "A helpful agent", "http://example.com");
        let json = card.to_json();
        assert!(json.contains("\"my-agent\""));
        assert!(json.contains("\"http://example.com\""));
        assert!(json.contains("\"capabilities\""));
    }

    #[test]
    fn to_json_omits_signature_when_none() {
        let card = AgentCard::new("a", "b", "c");
        let json = card.to_json();
        assert!(!json.contains("signature"));
        assert!(!json.contains("identity_key"));
    }

    #[test]
    fn canonical_bytes_excludes_signature_field() {
        let mut card = AgentCard::new("agent", "desc", "url");
        card.signature = Some("sig".into());
        card.identity_key = Some("key".into());
        let bytes = card.canonical_bytes();
        let text = String::from_utf8(bytes).unwrap();
        assert!(
            !text.contains("\"signature\""),
            "canonical bytes must not include signature"
        );
        assert!(
            text.contains("\"identity_key\""),
            "identity_key is part of canonical form"
        );
    }
}
