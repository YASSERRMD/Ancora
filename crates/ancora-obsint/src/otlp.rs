//! OTLP (OpenTelemetry Protocol) exporter configuration and transport types.
//! Supports both gRPC and HTTP/protobuf transports.

#[derive(Debug, Clone, PartialEq)]
pub enum OtlpTransport {
    Grpc,
    Http,
}

#[derive(Debug, Clone)]
pub struct OtlpConfig {
    pub endpoint: String,
    pub transport: OtlpTransport,
    pub headers: Vec<(String, String)>,
    pub timeout_ms: u64,
}

impl OtlpConfig {
    pub fn new_grpc(endpoint: impl Into<String>) -> Self {
        OtlpConfig {
            endpoint: endpoint.into(),
            transport: OtlpTransport::Grpc,
            headers: Vec::new(),
            timeout_ms: 5000,
        }
    }

    pub fn new_http(endpoint: impl Into<String>) -> Self {
        OtlpConfig {
            endpoint: endpoint.into(),
            transport: OtlpTransport::Http,
            headers: Vec::new(),
            timeout_ms: 5000,
        }
    }

    pub fn with_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((key.into(), value.into()));
        self
    }

    pub fn with_timeout_ms(mut self, ms: u64) -> Self {
        self.timeout_ms = ms;
        self
    }

    pub fn is_grpc(&self) -> bool {
        self.transport == OtlpTransport::Grpc
    }

    pub fn is_http(&self) -> bool {
        self.transport == OtlpTransport::Http
    }
}

#[derive(Debug, Clone)]
pub struct OtlpSpan {
    pub trace_id: [u8; 16],
    pub span_id: [u8; 8],
    pub name: String,
    pub start_ns: u64,
    pub end_ns: u64,
    pub attributes: Vec<(String, String)>,
    pub status_code: u32,
}

impl OtlpSpan {
    pub fn new(name: impl Into<String>, trace_id: [u8; 16], span_id: [u8; 8]) -> Self {
        OtlpSpan {
            trace_id,
            span_id,
            name: name.into(),
            start_ns: 0,
            end_ns: 0,
            attributes: Vec::new(),
            status_code: 0,
        }
    }

    pub fn duration_ns(&self) -> u64 {
        self.end_ns.saturating_sub(self.start_ns)
    }
}

#[derive(Debug, Clone)]
pub struct OtlpMetricPoint {
    pub name: String,
    pub value: f64,
    pub labels: Vec<(String, String)>,
    pub timestamp_ns: u64,
}

impl OtlpMetricPoint {
    pub fn new(name: impl Into<String>, value: f64) -> Self {
        OtlpMetricPoint {
            name: name.into(),
            value,
            labels: Vec::new(),
            timestamp_ns: 0,
        }
    }
}

/// Validates an OTLP endpoint string. Returns an error description if invalid.
pub fn validate_endpoint(endpoint: &str) -> Result<(), String> {
    if endpoint.is_empty() {
        return Err("endpoint must not be empty".to_string());
    }
    if !endpoint.starts_with("http://") && !endpoint.starts_with("https://") {
        return Err(format!(
            "endpoint '{}' must start with http:// or https://",
            endpoint
        ));
    }
    Ok(())
}
