use tonic::transport::ServerTlsConfig;

/// TLS configuration wrapping PEM-encoded certificate and private key bytes.
pub struct TlsConfig {
    cert_pem: Vec<u8>,
    key_pem: Vec<u8>,
}
