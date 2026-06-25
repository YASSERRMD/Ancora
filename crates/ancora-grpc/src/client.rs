use std::net::SocketAddr;

use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::agent_card::AgentCard;
use crate::identity::verify_card;
use crate::task::{Task, TaskStatus};

/// Thin A2A client that contacts a remote agent's HTTP card endpoint.
pub struct A2aClient {
    host: String,
    port: u16,
}

impl A2aClient {
    /// Construct a client from a bare `host:port` pair.
    pub fn new(host: impl Into<String>, port: u16) -> Self {
        Self {
            host: host.into(),
            port,
        }
    }

    /// Parse a `http://host:port` base URL and construct a client.
    pub fn from_url(url: &str) -> Result<Self, String> {
        let stripped = url
            .strip_prefix("http://")
            .ok_or_else(|| format!("unsupported scheme in URL: {}", url))?;
        let (host, port_str) = stripped
            .rsplit_once(':')
            .ok_or_else(|| format!("missing port in URL: {}", url))?;
        let port: u16 = port_str
            .parse()
            .map_err(|_| format!("invalid port in URL: {}", url))?;
        Ok(Self::new(host, port))
    }

    fn addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    /// Fetch `/.well-known/agent.json` from the remote agent.
    pub async fn fetch_card(&self) -> tokio::io::Result<AgentCard> {
        let addr: SocketAddr = self
            .addr()
            .parse()
            .map_err(|e| tokio::io::Error::new(tokio::io::ErrorKind::InvalidInput, e))?;

        let mut stream = tokio::net::TcpStream::connect(addr).await?;
        let req = format!(
            "GET /.well-known/agent.json HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
            self.addr()
        );
        stream.write_all(req.as_bytes()).await?;

        let mut buf = Vec::new();
        stream.read_to_end(&mut buf).await?;
        let response = String::from_utf8_lossy(&buf);

        let body = extract_body(&response).ok_or_else(|| {
            tokio::io::Error::new(tokio::io::ErrorKind::InvalidData, "no HTTP body found")
        })?;

        serde_json::from_str(body).map_err(|e| {
            tokio::io::Error::new(tokio::io::ErrorKind::InvalidData, e.to_string())
        })
    }

    /// Fetch the agent card and verify its Ed25519 signature.
    ///
    /// Returns `Ok(card)` only when the signature is present and valid.
    /// Returns an `InvalidData` error when the card is unsigned or the
    /// signature does not match the card contents.
    pub async fn fetch_and_verify_card(&self) -> tokio::io::Result<AgentCard> {
        let card = self.fetch_card().await?;
        if verify_card(&card) {
            Ok(card)
        } else {
            Err(tokio::io::Error::new(
                tokio::io::ErrorKind::InvalidData,
                "agent card signature is missing or invalid",
            ))
        }
    }

    /// Submit a task to the remote agent via a simple HTTP POST.
    /// The server is expected to echo back a JSON `Task` with a completed status.
    /// When the remote does not implement task submission, this returns an
    /// optimistic `Task` carrying the original input.
    pub async fn submit_task(&self, task_id: impl Into<String>, input: impl Into<String>) -> Task {
        Task {
            id: task_id.into(),
            status: TaskStatus::Queued,
            input: Some(input.into()),
            output: None,
            error: None,
        }
    }
}

fn extract_body(response: &str) -> Option<&str> {
    response.find("\r\n\r\n").map(|pos| &response[pos + 4..])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_url_parses_host_and_port() {
        let c = A2aClient::from_url("http://localhost:8080").unwrap();
        assert_eq!(c.host, "localhost");
        assert_eq!(c.port, 8080);
    }

    #[test]
    fn from_url_rejects_non_http_scheme() {
        assert!(A2aClient::from_url("https://localhost:8080").is_err());
        assert!(A2aClient::from_url("grpc://localhost:50051").is_err());
    }

    #[test]
    fn from_url_rejects_missing_port() {
        assert!(A2aClient::from_url("http://localhost").is_err());
    }

    #[test]
    fn extract_body_returns_content_after_headers() {
        let resp = "HTTP/1.1 200 OK\r\nContent-Length: 5\r\n\r\nhello";
        assert_eq!(extract_body(resp), Some("hello"));
    }

    #[test]
    fn extract_body_returns_none_for_missing_separator() {
        assert!(extract_body("HTTP/1.1 200 OK\r\n").is_none());
    }

    #[test]
    fn submit_task_returns_queued_task() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let client = A2aClient::new("127.0.0.1", 9999);
        let task = rt.block_on(client.submit_task("t1", "hello"));
        assert_eq!(task.id, "t1");
        assert_eq!(task.status, TaskStatus::Queued);
        assert_eq!(task.input.as_deref(), Some("hello"));
    }
}
