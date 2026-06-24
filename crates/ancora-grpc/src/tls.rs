use tonic::transport::{Identity, ServerTlsConfig};

/// TLS configuration wrapping PEM-encoded certificate and private key bytes.
pub struct TlsConfig {
    cert_pem: Vec<u8>,
    key_pem: Vec<u8>,
}

impl TlsConfig {
    /// Create a TLS config from PEM-encoded cert and key bytes.
    pub fn from_pem(cert_pem: Vec<u8>, key_pem: Vec<u8>) -> Self {
        Self { cert_pem, key_pem }
    }
}
