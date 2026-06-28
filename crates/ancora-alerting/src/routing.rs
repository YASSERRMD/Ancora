use crate::schema::FiredAlert;

/// A webhook endpoint receives fired alert payloads.
pub struct WebhookRouter {
    pub endpoint: String,
    pub sent: Vec<FiredAlert>,
}

impl WebhookRouter {
    pub fn new(endpoint: impl Into<String>) -> Self {
        Self { endpoint: endpoint.into(), sent: Vec::new() }
    }

    /// In production this would POST to the endpoint; here it accumulates into `sent`.
    pub fn route(&mut self, alert: FiredAlert) {
        self.sent.push(alert);
    }

    pub fn sent_count(&self) -> usize {
        self.sent.len()
    }
}
